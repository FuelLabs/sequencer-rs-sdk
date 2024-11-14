#!/usr/bin/env bash

# Generate protobuf files for rust api client

set -eo pipefail

(rm -f protos/*.rs)

# Generating rust API module
(buf generate --include-imports --verbose --template scripts/buf.gen.rust.yaml)
