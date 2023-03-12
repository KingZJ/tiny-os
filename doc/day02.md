### 切换rust 工具链 
rustup override set nightly

### 增加 x86_64-tiny_os.json 编译配置文件
cargo build --target x86_64-tiny_os.json

报错 error[E0463]: can't find crate for `core`

### 修改 .cargo/config.toml
增加内容
```
[unstable]
build-std = ["core", "compiler_builtins"]
```

再次执行 `cargo build --target x86_64-tiny_os.json`

提示 
.rustup/toolchains/nightly-x86_64-pc-windows-msvc/lib/rustlib/src/rust/Cargo.lock" does not exist, unable to build with the standard library, try:
        rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc
### 按照提示执行
`rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc`
再次进行编译已经正常

### 修改 .cargo/config.toml
```
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]

[build]
target = "x86_64-tiny_os.json"
```

### 此时可以直接使用 `cargo build` 进行编译


### 运行kernel
cargo install bootimage
rustup component add llvm-tools-preview
