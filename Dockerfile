# 编译容器
FROM --platform=$TARGETPLATFORM rust:alpine3.17 as builder

WORKDIR /usr/src

RUN USER=root cargo new chatgpt

RUN apk add musl-dev openssl openssl-dev pkgconfig upx

RUN echo -e "[source.crates-io]\nreplace-with = 'rsproxy'\n[source.rsproxy]\nregistry = 'https://rsproxy.cn/crates.io-index'\n[source.rsproxy-sparse]\nregistry = 'sparse+https://rsproxy.cn/index/'\n[registries.rsproxy]\nindex = 'https://rsproxy.cn/crates.io-index'\n[net]\ngit-fetch-with-cli = true" ~/.cargo/config

COPY Cargo.toml Cargo.lock /usr/src/chatgpt/

WORKDIR /usr/src/chatgpt

# 主要为了缓存依赖
RUN cargo build --release

COPY src /usr/src/chatgpt/src/

# 编译并使用upx压缩
RUN RUST_BACKTRACE=1 cargo build  --release && upx /usr/src/chatgpt/target/release/chatgpt

# 运行容器
FROM --platform=$TARGETPLATFORM alpine:3.17 AS runtime

WORKDIR /usr/local/chatgpt/

COPY --from=builder /usr/src/chatgpt/target/release/chatgpt /usr/local/chatgpt/

CMD ["/usr/local/chatgpt/chatgpt"]
