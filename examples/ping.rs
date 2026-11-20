//! Connects to Mica with mutual TLS and calls Ping, a good first check that your certificates and network path work.
//!
//!     cargo run --example ping -- https://api.example.mica.io:443 rootca.crt client.crt client.key

use mica_sdk::mica::member::ping::v1::{ping_service_client::PingServiceClient, PingRequest};
use mica_sdk::mtls::{connect, MtlsCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [target, ca, cert, key] = args.as_slice() else {
        eprintln!("usage: ping <https://host:port> <rootca.crt> <client.crt> <client.key>");
        std::process::exit(1);
    };
    let credentials = MtlsCredentials::from_files(ca, cert, key)?;
    let channel = connect(target.clone(), &credentials).await?;
    let response = PingServiceClient::new(channel).ping(PingRequest {}).await?.into_inner();
    println!("status={:?} service={} version={}", response.status(), response.service_type, response.build_version);
    Ok(())
}
