# Mica SDK for Rust

A Rust client library for the [Mica](https://mica.io) network. Mica's API is [gRPC](https://grpc.io) over mutual TLS;
the messages and clients in this repository are generated from the [public protos](https://github.com/1080network/proto)
with [prost](https://crates.io/crates/prost) and [tonic](https://crates.io/crates/tonic).

## One crate, a feature per role

Rust's way of installing only the code you need is Cargo features, so there is a single crate, `mica-sdk`, and you
enable the role or roles you play. Only their code is compiled.

| Feature | Use it if you are |
| --- | --- |
| `issuer` | an issuer |
| `acceptor` | an acceptor |

The shared messages, the `mica::member` services (such as `Ping`) and the mutual TLS helper are always included.

```toml
[dependencies]
mica-sdk = { version = "2.0.0-pre0", features = ["issuer"] }   # or "acceptor", or both
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Requires Rust 1.85 or later. Versions match the proto release (`v2.0.0-pre0` is `2.0.0-pre0`).

## Connecting

Mica uses mutual TLS. You need the root CA, and the client certificate and key Mica issued to you, as PEM files. The
server certificate is verified against the root CA and its host name, and the client presents your certificate.

```rust
use mica_sdk::mica::member::ping::v1::{ping_service_client::PingServiceClient, PingRequest};
use mica_sdk::mtls::{connect, MtlsCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = MtlsCredentials::from_files("rootca.crt", "client.crt", "client.key")?;
    let channel = connect("https://api.example.mica.io:443", &credentials).await?;

    let response = PingServiceClient::new(channel).ping(PingRequest {}).await?;
    println!("{:?}", response.into_inner().status());
    Ok(())
}
```

`connect` returns a channel you pass to any generated client, such as
`mica_sdk::mica::issuer::service::v2::issuer_to_mica_service_client::IssuerToMicaServiceClient`.

`examples/ping.rs` is a complete program that connects and calls `Ping`, a good first check that your certificates and
network path work:

```bash
cargo run --example ping -- https://api.example.mica.io:443 rootca.crt client.crt client.key
```

## Repository layout

| Path | Contents |
| --- | --- |
| `src/generated/` | **generated** prost and tonic code, one file per proto package, plus `mod.rs` which wires them into the module tree |
| `src/lib.rs`, `src/mtls.rs` | hand-written crate root and mutual TLS helper |
| `tests/mtls.rs` | a real mutual TLS call to an in-process server |
| `examples/` | runnable examples |
| `scripts/update-from-proto.sh` | regenerates the SDK for a proto release, then checks every feature combination and runs the tests |
| `scripts/gen-modules.py` | builds `src/generated/mod.rs` |

prost refers to other packages by relative paths, so the generated files must live in modules that mirror the proto
package hierarchy, and that is why this is one crate rather than one per role.

## Releasing (maintainers)

1. Run the **Generate SDKs** workflow in [1080network/proto](https://github.com/1080network/proto/actions) for the release tag.
2. Download its output: `gh run download <run-id> -R 1080network/proto -n generated-sdks-<version>`.
3. Run `scripts/update-from-proto.sh <version> <download-dir>`. It lays the generated code into the crate, sets the version, checks that every combination of role features compiles, runs the tests, and builds the package.
4. Open a pull request with the changes and merge it.
5. Publish to crates.io from the merged `main` with `cargo publish --all-features`. A crates.io token is needed.
