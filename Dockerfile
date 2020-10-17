FROM alpine:latest
LABEL maintainer="Jeremy Zheng"

RUN apk update
RUN apk add git curl vim zsh pwgen sudo build-base openssl-dev

# deploy user
RUN adduser -s /bin/bash -D deploy
RUN echo 'deploy ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/101-deploy
USER deploy

# https://github.com/ohmyzsh/ohmyzsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# https://www.rust-lang.org/tools/install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN echo 'source $HOME/.profile' >> $HOME/.zshrc

VOLUME /workspace
WORKDIR /workspace

ENV RUSTFLAGS="-C target-feature=-crt-static"
CMD ["/bin/zsh", "-l"]
