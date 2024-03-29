# 构建

## 本机构建

* ubuntu

```shell
sudo apt install -y pkg-config libudev-dev
sudo apt install -y aptitude
aptitude install libssl-dev
cargo build --release
```

## 交叉编译

[参考文档]

* https://www.cnblogs.com/007sx/p/15191400.html
* https://kerkour.com/rust-cross-compilation

### 环境配置

* os ubuntu 20.04

* 安装Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* 安装 cross

https://github.com/cross-rs/cross

```shell
cargo install -f cross
```

* 编译 window 可执行文件

```shell
brew install filosottile/musl-cross/musl-cross
```

```shell
cross build --release --target x86_64-pc-windows-gnu
```

* 编译 linux 可执行文件

* macos 请在 mac rust 环境下编译


## 参考文档

https://kerkour.com/rust-cross-compilation

