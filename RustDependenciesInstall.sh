#!/usr/bin/bash
MAX_RETRIES=1
RETRY_DELAY=3

for(( i=0; i<MAX_RETRIES; i++ )); do
    sudo apt-get update && rustup --version && \
    cargo install bootimage && \
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu && \
    rustup component add llvm-tools-preview && \
    sudo apt install -y qemu-system-x86
    if [ $? -eq 0 ]; then
        echo "---------------------------------SUCCESS BUILDING RUST DEPENDENCIES FOR crustyOS---------------------------------"
        break
    fi
    echo "DEPENDENCIES FAILED TO INSTALL, RETRYING $i MORE TIME IN $RETRY_DELAY SECONDS"
    sleep $RETRY_DELAY
done
