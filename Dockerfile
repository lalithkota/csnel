# com
FROM alpine
RUN apk add bash curl gcc g++ make libgcc libc-dev git
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /rustup-init.sh
RUN chmod +x /rustup-init.sh
RUN /rustup-init.sh -y --default-toolchain nightly
RUN echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $HOME/.bashrc
RUN /root/.cargo/bin/rustup component add llvm-tools-preview
RUN /root/.cargo/bin/rustup component add rust-src
RUN /root/.cargo/bin/cargo install bootimage

ADD . /csnel-src

# RUN cd /csnel-src ; /root/.cargo/bin/cargo build; /root/.cargo/bin/cargo bootimage
RUN cp -r /csnel-src/enduser /output-dir
RUN cd /output-dir ; /root/.cargo/bin/cargo build; /root/.cargo/bin/cargo bootimage

RUN mkdir /mnt-dir

RUN chmod +x /csnel-src/docker-entrypoint.sh
ENTRYPOINT /bin/bash -c /csnel-src/docker-entrypoint.sh
