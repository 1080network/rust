#!/bin/bash
# Regenerate the SDK from the output of the "Generate SDKs" workflow in https://github.com/1080network/proto.
#
# Usage: scripts/update-from-proto.sh <version> <generated-dir>
#   <version>        release tag the code was generated from: vX.Y.Z or vX.Y.Z-preN
#   <generated-dir>  the downloaded workflow artifact (the directory containing rust/)
#
# Download the generated code for a release with:
#   gh run download <run-id> -R 1080network/proto -n generated-sdks-<version>
set -euo pipefail

VERSION=${1:?usage: $0 <version> <generated-dir>}
GEN=${2:?usage: $0 <version> <generated-dir>}
ROOT=$(cd "$(dirname "$0")/.." && pwd)

if [[ ! "$VERSION" =~ ^v([0-9]+\.[0-9]+\.[0-9]+(-pre[0-9]+)?)$ ]]; then
    echo "version '$VERSION' must look like v1.2.3 or v1.2.3-pre0" >&2
    exit 1
fi
CARGO_VERSION=${BASH_REMATCH[1]}   # Cargo versions drop the leading v
if [[ ! -d "$GEN/rust/mica/issuer" ]]; then
    echo "'$GEN' does not look like generated output (expected rust/mica/issuer)" >&2
    exit 1
fi

echo "Laying out $VERSION ($CARGO_VERSION) from $GEN"
# Generated code only: hand-written code (lib.rs, mtls.rs, tests, examples) is left alone.
rm -rf "$ROOT/src/generated"
mkdir -p "$ROOT/src/generated"
# prost writes one file per proto package; they are flattened and wired together by gen-modules.py
find "$GEN/rust" -name '*.rs' -exec cp {} "$ROOT/src/generated/" \;
"$ROOT/scripts/gen-modules.py" "$ROOT/src/generated"

sed -i.bak -E "s/^version = \".*\"/version = \"$CARGO_VERSION\"/" "$ROOT/Cargo.toml" && rm "$ROOT/Cargo.toml.bak"
sed -i.bak -E "s/mica-sdk = \{ version = \"[^\"]*\"/mica-sdk = { version = \"$CARGO_VERSION\"/" "$ROOT/README.md" "$ROOT/src/lib.rs" && rm "$ROOT/README.md.bak" "$ROOT/src/lib.rs.bak"

cd "$ROOT"
# Every combination of roles must compile, not just all of them
cargo check --no-default-features
cargo check --features issuer
cargo check --features acceptor
cargo test --all-features
cargo package --all-features --allow-dirty --no-verify >/dev/null
echo "Done ($CARGO_VERSION)."
