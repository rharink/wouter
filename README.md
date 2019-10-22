Wouter
------

### Cross compile for arm7 (raspberry pi)
Add the std library for target
```bash
rustup target add armv7-unknown-linux-gnueabihf
```

Install cross compilation tools
```bash
apt install gcc-multilib-arm-linux-gnueabihf libc6-armhf-cross libc6-dev-armhf-cross
```

Update the .cargo/config to reflect the correct linker
```toml
#.cargo/config
[target.arm7-unknown-linux-gnueabifh]
linker = "arm-linux-gnueabihf-gcc"
```

Build the project for Pi
```ba
cargo build --target=armv7-unknown-linux-gnueabihf
```