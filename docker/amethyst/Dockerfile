FROM debian

ARG CHANNEL
ARG MDBOOK_RELEASE=0.2.1

RUN apt-get update && \
    apt-get install -y \
    git \
    libasound2-dev \
    libx11-xcb-dev \
    libssl-dev \
    cmake \
    libfreetype6-dev \
    libexpat1-dev \
    libxcb1-dev \
    python3 \
    build-essential \
    libsdl2-dev \
    curl

COPY asound.conf /etc/asound.conf
RUN useradd -ms /bin/bash jenkins
RUN usermod -aG audio jenkins
USER jenkins

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${CHANNEL}

ENV PATH=/home/jenkins/.cargo/bin:$PATH

RUN rustup component add rustfmt

RUN cargo install mdbook --vers ${MDBOOK_RELEASE}