#!/bin/sh

REPO_DIR="$HOME/personal/dev/requiem/"
DEST_DIR="${REPO_DIR}/uefi/esp/EFI/BOOT/BOOTX64.efi"

cargo build

cp "${REPO_DIR}/target/x86_64-unknown-uefi/debug/uefi.efi" "${DEST_DIR}"

qemu-system-x86_64 -cpu host -enable-kvm -serial stdio -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:esp
