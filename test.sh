#!/bin/sh

export AX_INIT=
export AX_ARCH=riscv64
export AX_PLATFORM=riscv64-qemu-virt
export AX_SMP=1
export AX_MODE=release

TEST_NAME=rt_fork
BUILD_DIR=./target/riscv64gc-unknown-none-elf/release/
LD_SCRIPT=$BUILD_DIR/linker_riscv64-qemu-virt.lds
GLOBAL_CFG='--cfg=blk --cfg=bus="mmio" --cfg=block_dev="virtio-blk"'
export RUSTFLAGS="-C link-arg=-T$LD_SCRIPT -C link-arg=-no-pie $GLOBAL_CFG"

git clone git@github.com:shilei-massclouds/btp
make -C btp

rm -f ./disk.img
dd if=/dev/zero of=./disk.img bs=1M count=256
mkfs.ext2 ./disk.img

mkdir -p ./mnt
sudo mount ./disk.img ./mnt
sudo mkdir -p ./mnt/lib
sudo mkdir -p ./mnt/btp
sudo cp -r ./btp/build/riscv64/sbin/ ./mnt/btp/
sudo cp /usr/riscv64-linux-gnu/lib/ld-linux-riscv64-lp64d.so.1 ./mnt/lib/
sudo cp /usr/riscv64-linux-gnu/lib/libc.so.6 ./mnt/lib/
sudo cp /usr/riscv64-linux-gnu/lib/libm.so.6 ./mnt/lib/
sudo cp /usr/riscv64-linux-gnu/lib/libresolv.so.2 ./mnt/lib/
ls -l ./mnt/
sudo umount ./mnt
rm -rf ./mnt

cargo build --target riscv64gc-unknown-none-elf --release --manifest-path $TEST_NAME/Cargo.toml

rust-objcopy --binary-architecture=riscv64 $BUILD_DIR/$TEST_NAME --strip-all -O binary $BUILD_DIR/$TEST_NAME.bin

qemu-system-riscv64 -m 128M -smp 1 -machine virt -bios default -kernel $BUILD_DIR/$TEST_NAME.bin -append "init=/sbin/init" -nographic \
    -device virtio-blk-device,drive=disk0 \
    -drive id=disk0,if=none,format=raw,file=./disk.img \
    -D qemu.log -d in_asm,int,mmu,pcall,cpu_reset,guest_errors
