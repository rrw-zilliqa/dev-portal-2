# Same as zq2 uses, or the z2 build fails :-(
FROM rust:1.78.0-slim-bullseye as builder

ENV DEBIAN_FRONTEND=noninteractive
ENV NEEDRESTART_MODE=a

RUN apt-get update && apt-get dist-upgrade -y
RUN apt-get install -y python3 python3-pip python3-setuptools --no-install-recommends
RUN apt-get install -y protobuf-compiler build-essential libssl-dev pkg-config git cmake
RUN apt-get autoremove

# Nonsensical, but allows us to cache requirements.
RUN mkdir /build
COPY requirements.txt /build/requirements.txt
RUN pip3 install --no-cache-dir -r /build/requirements.txt

COPY .  /build

ENV DOC_SOURCE=docs
WORKDIR /build/zq1
RUN  mkdocs build -f mkdocs.zq2.yml
WORKDIR /build/docgen
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/docgen/target \
    --mount=type=cache,target=/build/cache/zq2 \
    cargo run /build

WORKDIR /build/zq2
ARG VERSION
ENV VERSION=$VERSION
RUN mkdocs build

FROM nginx:alpine-slim

RUN mkdir -p /usr/share/nginx/html/zilliqa1
RUN mkdir -p /usr/share/nginx/html/zilliqa2
COPY --from=builder --chown=nginx:nginx /build/zq1/site/. /usr/share/nginx/html/zilliqa1/.
COPY --from=builder --chown=nginx:nginx /build/zq2/site/. /usr/share/nginx/html/zilliqa2/.
COPY default.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
