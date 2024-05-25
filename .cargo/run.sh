#! /bin/sh
#
# This script will be executed by `cargo run`.

set -xe

LIMINE_GIT_URL="https://github.com/limine-bootloader/limine.git"

# Cargo passes the path to the built executable as the first argument.
KERNEL=$1

# Clone the `limine` repository if we don't have it yet.
if [ ! -d target/limine ]; then
    git clone $LIMINE_GIT_URL --depth=1 --branch v7.x-binary target/limine
fi

# Make sure we have an up-to-date version of the bootloader.
cd target/limine
git fetch
make
cd -

# Copy the needed files into an ISO image.
mkdir -p target/iso_root/boot/limine
cp $KERNEL target/iso_root/boot
cp limine.cfg target/limine/limine{-bios.sys,-bios-cd.bin,-uefi-cd.bin} target/iso_root/boot/limine/

mkdir -p target/iso_root/EFI/BOOT
cp -v target/limine/BOOTX64.EFI target/iso_root/EFI/BOOT/
cp -v target/limine/BOOTIA32.EFI target/iso_root/EFI/BOOT/

xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin                  \
        -no-emul-boot -boot-load-size 4 -boot-info-table               \
        --efi-boot boot/limine/limine-uefi-cd.bin                      \
        -efi-boot-part --efi-boot-image --protective-msdos-label       \
        target/iso_root -o $KERNEL.iso

target/limine/limine bios-install $KERNEL.iso

# Run the created image with QEMU.
echo "Running in debug mode." >&2
qemu-system-x86_64                                                    \
    -machine q35 -cpu EPYC -M smm=off                                 \
    -D target/log.txt -d int,guest_errors -no-reboot -no-shutdown     \
    -serial stdio                                                     \
    -serial file:target/fb_log.txt                                    \
    -m 4G                                                             \
    -cdrom $KERNEL.iso >&2
    # -s -S \
