#!/usr/bin/env bash

set -e

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
proto_dir="${script_dir}/../proto/protos"
repo_dir="${script_dir}/../"
tmp_mavsdk_build_dir="/tmp/mavsdk_submodule"

echo "Removing existing generated-mavsdk folder"
rm -rf ${script_dir}/../generated-mavsdk

echo "CMake configuring MAVSDK submodule to /tmp to get necessary binaries"
mkdir -p ${tmp_mavsdk_build_dir}
cmake -DBUILD_MAVSDK_SERVER=ON -B${tmp_mavsdk_build_dir} -H${repo_dir}/MAVSDK

third_party_dir="${tmp_mavsdk_build_dir}/third_party"
protoc_binary="${third_party_dir}/install/bin/protoc"
protoc_grpc_binary="${third_party_dir}/install/bin/grpc_cpp_plugin"

echo "Looking for ${protoc_binary}"
if ! command -v ${protoc_binary} > /dev/null; then
    echo "Falling back to looking for protoc in PATH"
    if ! protoc_binary="$(command -v protoc)"; then
        echo >&2 "No protoc binary found"
        exit 1
    fi
fi
echo "Found protoc ($(${protoc_binary} --version)): ${protoc_binary}"

echo "Looking for ${protoc_grpc_binary}"
if ! command -v ${protoc_grpc_binary} > /dev/null; then
echo "Falling back to looking for grpc_cpp_plugin in PATH"
    if ! protoc_grpc_binary="$(command -v grpc_cpp_plugin)"; then
        echo >&2 "No grpc_cpp_plugin binary found"
        exit 1
    fi
fi
echo "Found grpc_cpp_plugin: ${protoc_grpc_binary}"

function snake_case_to_camel_case {
    echo $1 | awk -v FS="_" -v OFS="" '{for (i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1'
}

command -v ${protoc_binary} > /dev/null && command -v ${protoc_grpc_binary} > /dev/null || {
    echo "-------------------------------"
    echo " Error"
    echo "-------------------------------"
}

echo "Installing protoc-gen-mavsdk locally into build folder"
python3 -m pip install --upgrade --target=${tmp_mavsdk_build_dir}/pb_plugins ${script_dir}/../proto/pb_plugins

protoc_gen_mavsdk="${tmp_mavsdk_build_dir}/pb_plugins/bin/protoc-gen-mavsdk"
export PYTHONPATH="${tmp_mavsdk_build_dir}/pb_plugins:${PYTHONPATH}"
echo "Using protoc_gen_mavsdk: ${protoc_gen_mavsdk}"

plugin_list_and_core=$(cd ${script_dir}/../proto/protos && ls -d */ | sed 's:/*$::')
plugin_list=$(cd ${script_dir}/../proto/protos && ls -d */ | sed 's:/*$::' | grep -v core)

tmp_output_dir="$(mktemp -d)"
template_path_plugin_h="${script_dir}/../templates/plugin_h"
template_path_plugin_cpp="${script_dir}/../templates/plugin_cpp"

plugins_file="${script_dir}/../generated-mavsdk/plugins.txt"
mkdir -p $(dirname ${plugins_file})
# Overwrite plugins file to be empty
> $plugins_file

for plugin in ${plugin_list_and_core}; do

    echo "Processing ${plugin}/${plugin}.proto"

    if [[ "${plugin}" == "core" ]]; then
        continue
    fi

    mkdir -p ${script_dir}/../generated-mavsdk/mavsdk/plugins/${plugin}/include/plugins/${plugin}
    ${protoc_binary} -I ${proto_dir} --custom_out=${tmp_output_dir} --plugin=protoc-gen-custom=${protoc_gen_mavsdk} --custom_opt="file_ext=h,template_path=${template_path_plugin_h}" ${proto_dir}/${plugin}/${plugin}.proto
    mv ${tmp_output_dir}/${plugin}/$(snake_case_to_camel_case ${plugin}).h ${script_dir}/../generated-mavsdk/mavsdk/plugins/${plugin}/include/plugins/${plugin}/${plugin}.h

    ${protoc_binary} -I ${proto_dir} --custom_out=${tmp_output_dir} --plugin=protoc-gen-custom=${protoc_gen_mavsdk} --custom_opt="file_ext=cpp,template_path=${template_path_plugin_cpp}" ${proto_dir}/${plugin}/${plugin}.proto
    mv ${tmp_output_dir}/${plugin}/$(snake_case_to_camel_case ${plugin}).cpp ${script_dir}/../generated-mavsdk/mavsdk/plugins/${plugin}/${plugin}.cpp

    echo "${plugin}" >> $plugins_file
done

echo "Automatically running fix_style.py on generated code"
python3 ${script_dir}/fix_style.py -j$(nproc) ${script_dir}/../generated-mavsdk/mavsdk || true

echo "Completed generating MAVSDK plugins from protos"