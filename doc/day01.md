`#![no_std]`  禁止标准库，标准库依赖libc，而libc依赖操作系统
`#![no_main]` 禁止rust语言运行时，默认入口


rustup target add thumbv7em-none-eabihf

## 编译为裸机程序， 以裸机为目标
1. cargo build --target thumbv7em-none-eabihf

## 以本地操作系统为目标进行编译
### windows
2. cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
3. cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:windows"

### linux 
cargo rustc -- -C link-arg=-nostartfiles

### mac
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
将以上配置写到 .cargo/config.toml 中，可使用 cargo build 在本地操作系统进行编译，而不在报错

