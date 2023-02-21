#!/usr/bin/env bash

#
# Copyright (C) 2023 Asim Ihsan
# SPDX-License-Identifier: AGPL-3.0-only
#
# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU Affero General Public License as published by the Free
# Software Foundation, version 3.
#
# This program is distributed in the hope that it will be useful, but WITHOUT ANY
# WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
# PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License along
# with this program. If not, see <https://www.gnu.org/licenses/>
#

set -euxo pipefail

(cd rust && cargo test)
(cd rust && cargo build --release --target wasm32-unknown-unknown)
(cd rust-wasm-bindgen && cargo build --release --target wasm32-unknown-unknown)
(cd web && npm run build)
