set windows-shell := ["powershell.exe", "-c"]

[private]
default:
    @just --list --unsorted

build-library:
    @cargo build --release

list-examples:
    @python3 ./scripts/list-examples.py

build-example example:
    @cargo build --release --example {{example}}
    @#@arm-none-eabi-objdump target/armv7a-none-eabi/release/examples/{{example}} -S > temporary/disassembly/{{example}}.S
    @#@arm-none-eabi-readelf target/armv7a-none-eabi/release/examples/{{example}} -h

debug example: (build-example example)
    @arm-none-eabi-gdb -x scripts/run.gdb target/armv7a-none-eabi/release/examples/{{example}}

run-on-board:
    @xsct scripts/run_on_board.tcl
    @#xsct -interactive scripts/run_on_board.tcl

build-all-examples:
    @just build-example hello
    @just build-example gpio
    @just build-example leds
    @just build-example timer
    @just build-example dma
    @just build-example uart_interaction

doc open="--open":
    @cargo doc --release {{open}}
