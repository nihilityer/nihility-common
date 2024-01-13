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
use tracing::debug;

use crate::error::{NihilityCommonError, WrapResult};

static PRIVATE_KEY: OnceLock<RsaPrivateKey> = OnceLock::new();
static PUBLIC_KEY_MAP: OnceLock<HashMap<String, RsaPublicKey>> = OnceLock::new();
static SUBMODULE_AUTH_ID: OnceLock<String> = OnceLock::new();

const BIT_SIZE: usize = 700;
const CORE_PRIVATE_KEY_FILE_NAME: &str = "private.pem";
const CORE_PUBLIC_KEY_FILE_NAME: &str = "public.pem";
pub const AUTHENTICATION_ERROR_MESSAGE: &str = "Authentication Error";

pub trait Signature: Serialize {
    fn get_sign(&self) -> &Vec<u8>;
    fn set_sign(&mut self, sign: Vec<u8>);
}

pub fn submodule_authentication_core_init<P: AsRef<Path>>(
    id: String,
    core_public_key_path: P,
) -> WrapResult<RsaPublicKey> {
    let public_key = RsaPublicKey::read_public_key_pem_file(core_public_key_path)?;
    PUBLIC_KEY_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(id.to_string(), public_key);
        map
    });
    SUBMODULE_AUTH_ID.get_or_init(|| id);
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

pub fn get_public_key(auth_id: &String) -> WrapResult<&RsaPublicKey> {
    match PUBLIC_KEY_MAP
        .get()
        .expect("Public Key Map Not Init")
        .get(auth_id)
    {
        None => Err(NihilityCommonError::AuthId),
        Some(public_key) => Ok(public_key),
    }
}

pub fn verify<T: Signature>(entity: &mut T, buf: &mut [u8]) -> bool {
    match PRIVATE_KEY
        .get()
        .expect("Private Not Init")
        .decrypt(Pkcs1v15Encrypt, &entity.get_sign())
    {
        Ok(sign_data) => match String::from_utf8(sign_data) {
            Ok(sign) => {
                let parts: Vec<&str> = sign.as_str().split("|").collect();
                debug!("verify split result: {:?}", &parts);
                entity.set_sign(parts[0].as_bytes().into());
                parts[1].to_string().eq(&hex::encode(Sha256::digest(
                    postcard::to_slice(&entity, buf).expect("Encode Entity Error"),
                )))
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
    public_key: &RsaPublicKey,
    buf: &mut [u8],
) -> WrapResult<()> {
    entity.set_sign(auth_id.as_bytes().into());
    entity.set_sign(
        public_key
            .encrypt(
                &mut rand::thread_rng(),
                Pkcs1v15Encrypt,
                &(format!(
                    "{}|{}",
                    auth_id,
                    hex::encode(Sha256::digest(postcard::to_slice(&entity, buf)?))
                ))
                .as_bytes()[..],
            )
            .expect("Failed To Encrypt"),
    );
    Ok(())
}
