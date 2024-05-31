#! /bin/sh
#
# This script will be executed by `cargo run`.

set -xe

# Cargo passes the path to the built executable as the first argument.
KERNEL=$1

# Run the created image with QEMU.
echo "Running in debug mode." >&2
qemu-system-x86_64                              \
    -machine q35 -cpu EPYC                      \
    -D target/log.txt -M smm=off                \
    -s                                          \
    -serial stdio                               \
    -serial file:target/fb_log.tx               \
    -m 4G                                       \
    -cdrom $KERNEL.iso >&2
    #-d int,guest_errors -no-reboot -no-shutdown     \
