# 构建

## 本机构建

* ubuntu

```shell
sudo apt install pkg-config
sudo apt-get install libudev-dev
cargo build --release
```

## 交叉编译

[参考文档](https://www.cnblogs.com/007sx/p/15191400.html)

### 环境配置

* 安装

```shell
rustup target add x86_64-unknown-linux-musl
```

```shell
brew install filosottile/musl-cross/musl-cross
```