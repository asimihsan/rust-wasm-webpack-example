#!/usr/bin/env bash

set -euxo pipefail

 (cd rust && cargo test)
(cd web && npm run build)
