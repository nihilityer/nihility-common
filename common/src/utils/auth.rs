use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::OnceLock;

use rsa::pkcs8::{EncodePublicKey, LineEnding};
use rsa::{
    pkcs8::DecodePrivateKey, pkcs8::DecodePublicKey, pkcs8::EncodePrivateKey, Pkcs1v15Encrypt,
    RsaPrivateKey, RsaPublicKey,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;
use tracing::debug;
use uuid::Uuid;

use crate::entity::response::ResponseEntity;
use crate::error::{NihilityCommonError, WrapResult};
use crate::ModuleOperate;

static PRIVATE_KEY: OnceLock<RsaPrivateKey> = OnceLock::new();
static PUBLIC_KEY_MAP: OnceLock<Mutex<HashMap<String, RsaPublicKey>>> = OnceLock::new();
static SUBMODULE_AUTH_ID: OnceLock<String> = OnceLock::new();

const BIT_SIZE: usize = 2000;
const CORE_PRIVATE_KEY_FILE_NAME: &str = "private.pem";
const CORE_PUBLIC_KEY_FILE_NAME: &str = "public.pem";
pub const AUTHENTICATION_ERROR_MESSAGE: &str = "Authentication Error";
pub const SUBMODULE_PUBLIC_KEY: &str = "public_key";

pub trait Signature: Serialize {
    fn get_sign(&self) -> &Vec<u8>;
    fn set_sign(&mut self, sign: Vec<u8>);
}

pub fn submodule_authentication_core_init<P: AsRef<Path>>(
    module_name: String,
    core_public_key_path: P,
) -> WrapResult<RsaPublicKey> {
    let public_key = RsaPublicKey::read_public_key_pem_file(core_public_key_path)?;
    PUBLIC_KEY_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(module_name, public_key);
        Mutex::new(map)
    });
    Ok(RsaPublicKey::from(PRIVATE_KEY.get_or_init(|| {
        let mut rng = rand::thread_rng();
        match RsaPrivateKey::new(&mut rng, BIT_SIZE) {
            Ok(private_key) => private_key,
            Err(e) => panic!("Private Key Init Error: {}", e),
        }
    })))
}

pub fn core_authentication_core_init<P: AsRef<Path>>(key_dir: P) -> WrapResult<()> {
    let dir_path = key_dir.as_ref();
    let private_key_path = dir_path.join(CORE_PRIVATE_KEY_FILE_NAME);
    let public_key_path = dir_path.join(CORE_PUBLIC_KEY_FILE_NAME);
    if dir_path.exists() {
        let private_key = RsaPrivateKey::read_pkcs8_pem_file(private_key_path)?;
        PRIVATE_KEY.get_or_init(|| private_key);
    } else {
        create_dir_all(dir_path)?;
        let private_key = PRIVATE_KEY.get_or_init(|| {
            let mut rng = rand::thread_rng();
            match RsaPrivateKey::new(&mut rng, BIT_SIZE) {
                Ok(private_key) => private_key,
                Err(e) => panic!("Private Key Init Error: {}", e),
            }
        });
        private_key.write_pkcs8_pem_file(private_key_path, LineEnding::default())?;
        let public_key = RsaPublicKey::from(private_key);
        public_key.write_public_key_pem_file(public_key_path, LineEnding::default())?;
    }
    PUBLIC_KEY_MAP.get_or_init(|| Mutex::new(HashMap::new()));
    Ok(())
}

pub fn set_entity_submodule_sign<T: Signature>(mut entity: T) -> T {
    entity.set_sign(
        SUBMODULE_AUTH_ID
            .get()
            .expect("Auth Id Not Init")
            .as_bytes()
            .into(),
    );
    entity
}

pub async fn set_module_operate_register_info(
    module_operate: &mut ModuleOperate,
) -> WrapResult<String> {
    let module_name = String::from_utf8_lossy(module_operate.get_sign()).to_string();
    let uuid = Uuid::new_v4().to_string();
    let sign = format!("{}|{}", module_name, &uuid);
    module_operate.set_sign(sign.as_bytes().into());
    match &module_operate.info {
        None => Err(NihilityCommonError::ConfigFieldMissing),
        Some(info) => match info.conn_params.conn_params.get(SUBMODULE_PUBLIC_KEY) {
            None => Err(NihilityCommonError::ConfigFieldMissing),
            Some(public_key_string) => {
                let public_key = RsaPublicKey::from_public_key_pem(public_key_string.as_str())?;
                let mut map = PUBLIC_KEY_MAP.get().unwrap().lock().await;
                map.insert(uuid.to_string(), public_key);
                Ok(uuid)
            }
        },
    }
}

pub fn get_module_operate_register_info(
    module_operate: &ModuleOperate,
) -> WrapResult<(String, String)> {
    let sign = String::from_utf8_lossy(module_operate.get_sign()).to_string();
    let signs: Vec<&str> = sign.split('|').collect();
    if signs.len() != 2 {
        return Err(NihilityCommonError::AuthId);
    }
    Ok((signs[0].to_string(), signs[1].to_string()))
}

pub async fn submodule_resister_success(resp: &mut ResponseEntity) -> WrapResult<()> {
    let register_id = String::from_utf8_lossy(resp.get_sign()).to_string();
    SUBMODULE_AUTH_ID.get_or_init(|| register_id.clone());
    let mut map = PUBLIC_KEY_MAP
        .get()
        .expect("Public Key Map Not Init")
        .lock()
        .await;
    let mut key = None;
    for (_, public_key) in map.iter() {
        key = Some(public_key.clone());
    }
    map.insert(register_id, key.unwrap().clone());
    Ok(())
}

pub async fn get_public_key(auth_id: &String) -> WrapResult<RsaPublicKey> {
    match PUBLIC_KEY_MAP
        .get()
        .expect("Public Key Map Not Init")
        .lock()
        .await
        .get(auth_id)
    {
        None => Err(NihilityCommonError::AuthId),
        Some(public_key) => Ok(public_key.clone()),
    }
}

pub fn verify<T: Signature>(entity: &mut T, buf: &mut [u8]) -> bool {
    match PRIVATE_KEY
        .get()
        .expect("Private Not Init")
        .decrypt(Pkcs1v15Encrypt, entity.get_sign())
    {
        Ok(sign_data) => match String::from_utf8(sign_data) {
            Ok(sign) => {
                let parts: Vec<&str> = sign.split('|').collect();
                debug!("verify split result: {:?}", &parts);
                entity.set_sign(parts[0].as_bytes().into());
                let sha256 = hex::encode(Sha256::digest(
                    postcard::to_slice(&entity, buf).expect("Encode Entity Error"),
                ));
                debug!("verify Sha256: {:?}", &sha256);
                parts[1].to_string().eq(&sha256)
            }
            Err(e) => {
                debug!("Decode Data To String Error: {}", e);
                false
            }
        },
        Err(e) => {
            debug!("Decrypt Error: {}", e);
            false
        }
    }
}

pub fn signature<T: Signature>(
    entity: &mut T,
    auth_id: &String,
    public_key: RsaPublicKey,
    buf: &mut [u8],
) -> WrapResult<()> {
    debug!("Signature Auth Id: {:?}", auth_id);
    entity.set_sign(auth_id.as_bytes().into());
    let sha256 = hex::encode(Sha256::digest(postcard::to_slice(&entity, buf)?));
    debug!("Signature Sha256: {:?}", &sha256);
    entity.set_sign(
        public_key
            .encrypt(
                &mut rand::thread_rng(),
                Pkcs1v15Encrypt,
                (format!("{}|{}", auth_id, sha256)).as_bytes(),
            )
            .expect("Failed To Encrypt"),
    );
    Ok(())
}
