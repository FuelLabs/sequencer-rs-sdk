# Generate protobuf files for rust api client
set -eo pipefail

git submodule update --init

(rm -Rf src/protos)

# Generating rust API module
# Requires `buf` to be installed
# `brew install buf`
(buf generate --include-imports --verbose --template buf.gen.rust.yaml)

rustfmt +nightly --edition 2021 src/protos/*.rs