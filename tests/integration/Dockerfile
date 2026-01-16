FROM debian:trixie

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libcurl4-openssl-dev \
    libxml2-dev \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /root

RUN curl --location --remote-name https://github.com/Orange-OpenSource/hurl/releases/download/7.0.0/hurl_7.0.0_amd64.deb
RUN apt-get install /root/hurl_7.0.0_amd64.deb
RUN rm hurl_7.0.0_amd64.deb
COPY tests/integration/couic.toml /tmp
COPY tests/integration/run.sh /tmp
