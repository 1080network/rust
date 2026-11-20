//! Makes real mutual TLS Ping calls to an in-process server, using certificates generated for the test.

use mica_sdk::mica::member::ping::v1::{
    ping_service_client::PingServiceClient,
    ping_service_server::{PingService, PingServiceServer},
    ping_response::Status,
    PingRequest, PingResponse,
};
use mica_sdk::mtls::{connect, MtlsCredentials};
use rcgen::{BasicConstraints, CertificateParams, IsCa, Issuer, KeyPair};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status as GrpcStatus};

struct Pinger;

#[tonic::async_trait]
impl PingService for Pinger {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingResponse>, GrpcStatus> {
        Ok(Response::new(PingResponse { status: Status::Success as i32, service_type: "test".into(), ..Default::default() }))
    }
}

struct Authority {
    params: CertificateParams,
    key: KeyPair,
    pem: String,
}

fn authority(name: &str) -> Authority {
    let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
    params.distinguished_name.push(rcgen::DnType::CommonName, name);
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    let key = KeyPair::generate().unwrap();
    let pem = params.self_signed(&key).unwrap().pem();
    Authority { params, key, pem }
}

/// A certificate for `localhost` and 127.0.0.1 signed by `ca`: (certificate PEM, key PEM).
fn leaf(name: &str, ca: &Authority) -> (String, String) {
    let mut params = CertificateParams::new(vec!["localhost".to_string()]).unwrap();
    params.subject_alt_names.push(rcgen::SanType::IpAddress("127.0.0.1".parse().unwrap()));
    params.distinguished_name.push(rcgen::DnType::CommonName, name);
    let key = KeyPair::generate().unwrap();
    let issuer = Issuer::new(ca.params.clone(), &ca.key);
    let cert = params.signed_by(&key, &issuer).unwrap();
    (cert.pem(), key.serialize_pem())
}

#[tokio::test(flavor = "multi_thread")]
async fn ping_succeeds_over_mutual_tls_and_a_server_from_an_unknown_authority_is_rejected() {
    let ca = authority("test ca");
    let (server_cert, server_key) = leaf("server", &ca);
    let (client_cert, client_key) = leaf("client", &ca);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let tls = ServerTlsConfig::new()
        .identity(Identity::from_pem(server_cert, server_key))
        .client_ca_root(Certificate::from_pem(&ca.pem)); // requires and verifies a client certificate
    tokio::spawn(
        Server::builder()
            .tls_config(tls)
            .unwrap()
            .add_service(PingServiceServer::new(Pinger))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );
    let target = format!("https://localhost:{port}");

    let credentials = MtlsCredentials {
        certificate_authorities: ca.pem.clone().into_bytes(),
        certificate: client_cert.clone().into_bytes(),
        key: client_key.clone().into_bytes(),
    };
    let channel = connect(&target, &credentials).await.unwrap();
    let response = PingServiceClient::new(channel).ping(PingRequest {}).await.unwrap();
    assert_eq!(response.into_inner().status(), Status::Success);

    let other_ca = authority("other ca");
    let untrusting = MtlsCredentials {
        certificate_authorities: other_ca.pem.into_bytes(),
        certificate: client_cert.into_bytes(),
        key: client_key.into_bytes(),
    };
    let rejected = match connect(&target, &untrusting).await {
        Err(_) => true,
        Ok(channel) => PingServiceClient::new(channel).ping(PingRequest {}).await.is_err(),
    };
    assert!(rejected, "a server certificate from an unknown authority must be rejected");
}

#[tokio::test]
async fn an_invalid_target_is_an_error() {
    let credentials = MtlsCredentials { certificate_authorities: vec![], certificate: vec![], key: vec![] };
    assert!(connect("not a uri", &credentials).await.is_err());
}
