//! Connect to the Mica network with mutual TLS.
//!
//! Every connection to Mica uses mutual TLS: the client presents a certificate issued by Mica and verifies Mica's
//! server certificate (including its host name) against the root certificate authority Mica provides.

use std::io;
use std::path::Path;

use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

/// PEM material for a mutual TLS connection to Mica.
#[derive(Clone)]
pub struct MtlsCredentials {
    /// Root certificate(s) used to verify Mica's server certificate.
    pub certificate_authorities: Vec<u8>,
    /// The client certificate chain issued by Mica.
    pub certificate: Vec<u8>,
    /// The private key for `certificate`.
    pub key: Vec<u8>,
}

impl MtlsCredentials {
    /// Reads the credentials from PEM files on disk.
    pub fn from_files(
        certificate_authorities: impl AsRef<Path>,
        certificate: impl AsRef<Path>,
        key: impl AsRef<Path>,
    ) -> io::Result<Self> {
        Ok(Self {
            certificate_authorities: std::fs::read(certificate_authorities)?,
            certificate: std::fs::read(certificate)?,
            key: std::fs::read(key)?,
        })
    }

    /// The tonic TLS configuration for these credentials.
    pub fn tls_config(&self) -> ClientTlsConfig {
        ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(&self.certificate_authorities))
            .identity(Identity::from_pem(&self.certificate, &self.key))
    }
}

/// An error opening a channel to Mica.
#[derive(Debug)]
pub enum ConnectError {
    /// The target is not a valid URI, for example `https://api.example.mica.io:443`.
    InvalidTarget(tonic::codegen::http::uri::InvalidUri),
    /// The TLS configuration was rejected or the connection could not be established.
    Transport(tonic::transport::Error),
}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTarget(e) => write!(f, "invalid target: {e}"),
            Self::Transport(e) => write!(f, "connection failed: {e}"),
        }
    }
}

impl std::error::Error for ConnectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTarget(e) => Some(e),
            Self::Transport(e) => Some(e),
        }
    }
}

/// Opens a channel to `target` (for example `https://api.example.mica.io:443`) with mutual TLS.
///
/// Pass the channel to any generated client, for example `PingServiceClient::new(channel)`.
pub async fn connect(target: impl Into<String>, credentials: &MtlsCredentials) -> Result<Channel, ConnectError> {
    Channel::from_shared(target.into())
        .map_err(ConnectError::InvalidTarget)?
        .tls_config(credentials.tls_config())
        .map_err(ConnectError::Transport)?
        .connect()
        .await
        .map_err(ConnectError::Transport)
}
