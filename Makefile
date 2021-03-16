TARGET		:= riscv64imac-unknown-none-elf
MODE		:= debug
KERNEL_FILE	:= target/$(TARGET)/$(MODE)/os
BIN_FILE	:= target/$(TARGET)/$(MODE)/kernel.bin

OBJDUMP		:= rust-objdump --arch-name=riscv64
OBJCOPY		:= rust-objcopy --binary-architecture=riscv64

.PHONY: doc kernel build clean qemu run

# build 输出为二进制文件
build: $(BIN_FILE)

# 通过 Rust 注释生成 os 文档
doc:
	@cargo doc --document-private-items

# 编译 kernel
kernel:
	@cargo build

# 编译 kernel 的二进制文件
$(BIN_FILE): kernel
	@$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $@

# 查看反汇编
asm:
	@$(OBJDUMP) -d $(KERNEL_FILE) | less

# 清理编译文件
clean:
	@cargo clean

# 运行 QEMU
qemu: build
	@qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file=$(BIN_FILE),addr=0x80200000

# 综合运行
run: build qemu