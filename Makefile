.PHONY: all build release image run serial hosted ordinals clean

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

# .cargo/config.toml pins the bare target for every invocation, so a plain
# `cargo build --features hosted` still compiles no_std and fails with thousands
# of missing-prelude errors that look like rot and are not. The host target has
# to be named explicitly. Use these rather than remembering that.
HOST := x86_64-unknown-linux-gnu

hosted:
	cargo build --features hosted --target $(HOST)

# The ordinal faithfulness guard, without QEMU. Boots the kernel on the host and
# fails loudly if the canonical ordinals have drifted from the Lean table.
# PID 1 is a perpetual event loop by design, so the boot is bounded and the kill
# is expected; only the guard line decides the exit status.
ordinals: hosted
	@timeout 60 ./target/$(HOST)/debug/momonados < /dev/null 2>&1 \
	   > .ordinals.log || true
	@if grep -aqi '44 values match' .ordinals.log; then \
	   grep -am1 'Ordinal faithfulness' .ordinals.log; rm -f .ordinals.log; \
	 elif grep -aqi 'ordinal drift' .ordinals.log; then \
	   echo "ORDINAL DRIFT DETECTED"; \
	   grep -aE 'DRIFT' .ordinals.log | tail -3; rm -f .ordinals.log; exit 1; \
	 else \
	   echo "NO BOOT OUTPUT — the guard did not speak, which is not a pass"; \
	   tail -3 .ordinals.log; rm -f .ordinals.log; exit 3; \
	 fi

clean:
	cargo clean
	rm -rf target/uefi-boot .ovmf_vars.fd
