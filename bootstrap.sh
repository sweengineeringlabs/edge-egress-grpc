#!/usr/bin/env bash
set -euo pipefail
cargo build -p swe-edge-egress-grpc
cargo test  -p swe-edge-egress-grpc
