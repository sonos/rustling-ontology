FROM ubuntu:16.04
RUN apt-get update && apt-get upgrade -y && apt-get clean
RUN apt-get update && apt-get install -y python-setuptools python-wheel
RUN apt-get install -y gcc g++ curl wget unzip git pkg-config libssl-dev libssh2-1-dev cmake\
    && apt-get clean

# Add user jenkins to the image
RUN adduser --quiet --home /build/ build

RUN git clone https://github.com/raspberrypi/tools /opt/pitools --depth 1

USER build
WORKDIR /build

#install rust
RUN HOME=/build curl https://sh.rustup.rs -sSf | HOME=/build sh -s -- -y

#setup path with cargo & protoc
RUN echo 'PATH=/build/.cargo/bin:$PATH' >> /build/.bashrc

RUN /build/.cargo/bin/cargo install dinghy

RUN /build/.cargo/bin/rustup target install armv7-unknown-linux-gnueabihf \
    &&  /build/.cargo/bin/rustup target install arm-unknown-linux-gnueabihf

RUN echo '[target.arm-unknown-linux-gnueabihf]\n\
linker = "/opt/pitools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"\n\
\n\
[target.armv7-unknown-linux-gnueabihf]\n\
linker = "/opt/pitools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"\n\
'>> /build/.cargo/config


RUN echo 'export TARGET_CC=/opt/pitools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc' >> /build/.bashrc

RUN ln -s /pypirc /build/.pypirc
RUN ln -s /dinghy.toml /build/.dinghy.toml
RUN ln -s /ssh-conf /build/.ssh

WORKDIR /build/workdir
