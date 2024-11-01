#
# Copyright 2023, Colias Group, LLC
#
# SPDX-License-Identifier: BSD-2-Clause
#

export BUILD ?= $(abspath build)
BOARD ?= qemu_virt_aarch64
MICROKIT_SDK ?= /Users/ivanv/ts/microkit-sdk-1.4.1

build_dir := $(BUILD)/$(BOARD)

.PHONY: none
none:

.PHONY: clean
clean:
	rm -rf $(BUILD)

$(build_dir):
	mkdir -p $@

microkit_board := $(BOARD)
microkit_config := debug
microkit_sdk_config_dir := $(MICROKIT_SDK)/board/$(microkit_board)/$(microkit_config)

sel4_include_dirs := $(microkit_sdk_config_dir)/include

### Protection domains

crate = $(build_dir)/$(1).elf

define build_crate

$(crate): $(crate).intermediate

.INTERMDIATE: $(crate).intermediate
$(crate).intermediate: $(BUILD)/wasi_test.wasm
	SEL4_INCLUDE_DIRS=$(abspath $(sel4_include_dirs)) \
		cargo build \
			-Z unstable-options \
			-Z build-std=core,alloc,compiler_builtins \
			-Z build-std-features=compiler-builtins-mem \
			--target-dir $(build_dir)/target \
			--out-dir $(build_dir) \
			--target aarch64-sel4-microkit-minimal \
			--release \
			-p $(1)

endef

crate_names := \
	wasmi-interpreter \
	client-loader

crates := $(foreach crate_name,$(crate_names),$(call crate,$(crate_name)))

$(eval $(foreach crate_name,$(crate_names),$(call build_crate,$(crate_name))))

### WASM Rust crate

$(BUILD)/wasi_test.wasm: wasi_test/src/main.rs
	cargo build -p wasi_test --target wasm32-wasip2 --out-dir $(BUILD)

### Loader

system_description := wasm.system

loader := $(build_dir)/loader.img

$(loader): $(system_description) $(crates)
	$(MICROKIT_SDK)/bin/microkit \
		$< \
		--search-path $(build_dir) \
		--board $(microkit_board) \
		--config $(microkit_config) \
		-r $(build_dir)/report.txt \
		-o $@

.PHONY: build
build: $(loader)

### Run

ifeq ($(BOARD),qemu_virt_aarch64)

qemu_cmd := \
	qemu-system-aarch64 \
		-machine virt,virtualization=on -cpu cortex-a53 -m size=2G \
		-serial mon:stdio \
		-nographic \
		-device loader,file=$(loader),addr=0x70000000,cpu-num=0

.PHONY: run
run: $(loader)
	$(qemu_cmd)

endif

.PHONY: doc
doc:
	SEL4_INCLUDE_DIRS=$(abspath $(sel4_include_dirs)) \
	cargo doc \
		-Z unstable-options \
		-Z build-std=core,alloc,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem \
		--target aarch64-sel4-microkit-minimal \
		--open