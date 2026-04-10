#!/bin/sh

REPO_DIR="~/personal/dev/requiem/"
DEST_DIR="${REPO_DIR}/uefi/esp/EFI/BOOT/BOOTX64.efi"

cargo build

cp "${REPO_DIR}/target/x86_64-unknown-uefi/debug/uefi.efi"

qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:esp
