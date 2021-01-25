use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use std::io::BufReader;
use std::sync::Arc;
use async_tls::TlsAcceptor;
use log::debug;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};

pub fn get_acceptor() -> Result<TlsAcceptor, Box<dyn Error>> {
    let cert_path = env::var("TLS_CERT_PATH")?;
    let cert = certs(&mut BufReader::new(File::open(cert_path)?))
        .map_err(|_| ioErr::new(ioErrKind::InvalidInput, "Invalid certificate"))?;

    let key_path = env::var("TLS_KEY_PATH")?;
    let mut key = rsa_private_keys(&mut BufReader::new(File::open(key_path)?))
        .map_err(|_| ioErr::new(ioErrKind::InvalidInput, "Invalid key"))?;

    let mut config = ServerConfig::new(NoClientAuth::new());
        config.set_single_cert(cert, key.remove(0))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    Ok(acceptor)
}
