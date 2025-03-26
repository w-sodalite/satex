FROM alpine:latest AS build-env

# 修改Alpine镜像地址
RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 设置rustup安装代理环境
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

# 安装依赖库
RUN apk update
RUN apk add -u --no-cache gcc
RUN apk add -u --no-cache musl-dev
RUN apk add -u --no-cache wget

# 安装rustup
RUN mkdir /tmp/run
RUN wget https://rsproxy.cn/rustup-init.sh
RUN sh rustup-init.sh -y --profile minimal --no-modify-path

# 链接rustup和cargo到/usr/local/bin
RUN ln -s $HOME/.cargo/bin/rustup /usr/local/bin/rustup
RUN ln -s $HOME/.cargo/bin/cargo /usr/local/bin/cargo

# 导入cargo配置
COPY proxy_config $HOME/.cargo/config

WORKDIR /satex

# 复制文件
COPY src ./src
COPY crates ./crates
COPY Cargo.toml .
COPY Cargo.lock .

# 编译
RUN cargo build --release

FROM alpine:latest

WORKDIR /satex

# 复制构建文件
COPY --from=build-env /satex/target/release/satex ./bin/

# 复制配置文件
COPY examples/docker/satex.yaml ./conf/
COPY examples/docker/static.yaml ./conf/
COPY examples/resources ./static/

# 暴露端口
EXPOSE 80

# 启动
ENTRYPOINT ["/satex/bin/satex", "-c", "/satex/conf/satex.yaml"]