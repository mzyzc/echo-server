use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Error as ioErr;
use std::io::ErrorKind as ioErrKind;
use std::sync::Arc;
use async_tls::TlsAcceptor;
use rustls::{Certificate, PrivateKey};
use rustls::{NoClientAuth, ServerConfig};
use rustls_pemfile;

// Create a TLS acceptor using a local certificate and key
pub async fn get_acceptor() -> Result<TlsAcceptor, Box<dyn Error>> {
    let cert = get_cert(
        &env::var("TLS_CERT_PATH")?
    )?;
    let key = get_key(
        &env::var("TLS_KEY_PATH")?
    )?;

    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(cert, key)?;

    let acceptor = TlsAcceptor::from(Arc::new(config));
    Ok(acceptor)
}

// Read a locally-stored certificate
fn get_cert(path: &str) -> Result<Vec<Certificate>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let cert = rustls_pemfile::certs(&mut reader)?
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect();
    
    Ok(cert)
}

// Read a locally-stored private key
fn get_key(path: &str) -> Result<PrivateKey, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    loop {
        let key = rustls_pemfile::read_one(&mut reader)?;
        match key {
            Some(rustls_pemfile::Item::RSAKey(k)) => return Ok(rustls::PrivateKey(k)),
            Some(rustls_pemfile::Item::PKCS8Key(k)) => return Ok(rustls::PrivateKey(k)),
            None => { return Err(Box::new(ioErr::new(ioErrKind::InvalidData, "Invalid key"))) }
            _ => { return Err(Box::new(ioErr::new(ioErrKind::InvalidData, "Invalid key"))) },
        }
    }
}