default:
    just --list --unsorted

build:
    cargo build --release

hello:
    cargo build --release --example hello

hello-asm:
    just hello
    arm-none-eabi-objdump target/armv7a-none-eabi/release/examples/hello -S
    arm-none-eabi-readelf target/armv7a-none-eabi/release/examples/hello -h

gpio:
    cargo build --release --example gpio

gpio-asm:
    just gpio
    arm-none-eabi-objdump target/armv7a-none-eabi/release/examples/gpio -S
    arm-none-eabi-readelf target/armv7a-none-eabi/release/examples/gpio -h
