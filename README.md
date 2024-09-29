# rustM32

Rust program on the STM32

## how to run

Clone the repository by running:

```
git clone https://github.com/jasonlmfong/rustM32.git
cd rustM32
```

<!-- ```
rustup update
rustup target install thumbv7m-none-eabi
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
``` -->

Compile the code and flash the binary by running the following:

```
cargo rustc --target thumbv7m-none-eabi --release -- -C link-arg=-Tlink.x
```

you should see the ELF file `target/thumbv7m-none-eabi/release/rustM32` created.

Build the binary by running:

```
sudo apt install binutils-arm-none-eabi
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/rustM32 rustM32.bin
```

Flash the binary

```
sudo apt install stlink-tools
st-flash write rustM32.bin 0x8000000
```

Ta-da!
