.PHONY: all build release image run serial clean

all: build

build:
	cargo build

release:
	cargo build --release

image: release
	bash build_bootimage.sh

run: image
	bash run.sh --serial

serial: image
	bash run.sh --serial

clean:
	cargo clean
	rm -rf target/uefi-boot .ovmf_vars.fd
