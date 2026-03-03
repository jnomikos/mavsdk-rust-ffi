#!/usr/bin/env bash

set -e
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
rust_ffi_output_dir="${script_dir}/../cxx/gen"
echo "Formatting generated Rust code in ${rust_ffi_output_dir}"
find ${rust_ffi_output_dir} -name "*.rs" -exec rustfmt --edition 2024 {} \;