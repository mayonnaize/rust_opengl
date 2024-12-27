# バージョンは適当
FROM rust:1.70 as rust_opengl

RUN apt-get update && apt-get install -y \
    sudo \
    git \
    curl \
    wget \
    unzip \
    zip \
    build-essential \
    python3 \
    python3-pip \
    python3-setuptools \
    python3-wheel \
    git \
    gdb \
    && rm -rf /var/lib/apt/lists/*

ARG USER=user
RUN groupadd -g 1000 -r $USER
RUN useradd -u 1000 -g 1000 --create-home -r $USER
#Change password
RUN echo "$USER:$USER" | chpasswd
#Make sudo passwordless
RUN mkdir -p /etc/sudoers.d && echo "${USER} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/90-$USER
# RUN echo "${USER} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/90-$USER
RUN usermod -aG sudo $USER
RUN usermod -aG plugdev $USER
USER $USER

WORKDIR /home/user
