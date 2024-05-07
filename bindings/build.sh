#!/bin/bash

set -e

lib_name="hydra_dx"
output_dir="./xcframework"
release_dir="./target"

rm -rf $output_dir
mkdir -p $output_dir

# We need single folder with headers to put module map within it
headers_temp_dir="./hydra-dx/headers"
mkdir -p $headers_temp_dir
cp "./hydra-dx/Generated/SwiftBridgeCore.h" $headers_temp_dir
cp "./hydra-dx/Generated/hydra-dx/hydra-dx.h" $headers_temp_dir

# Create module.modulemap file
cat <<EOF >$headers_temp_dir/module.modulemap
module ${lib_name} {
    header "SwiftBridgeCore.h"
    header "hydra-dx.h"
    export *
}
EOF

xcodebuild -create-xcframework \
    -library $release_dir/aarch64-apple-ios/release/lib${lib_name}.a \
    -headers $headers_temp_dir \
    -library $release_dir/x86_64-apple-ios/release/lib${lib_name}.a \
    -headers $headers_temp_dir \
    -output $output_dir/${lib_name}.xcframework

echo "XCFramework created at $output_dir/${lib_name}.xcframework"

rm -rf $headers_temp_dir

echo "Cleanup finished"
