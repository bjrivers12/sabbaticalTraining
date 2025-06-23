FROM gcc:latest

RUN apt-get update && \
    apt-get install -y \ 
    build-essential \
    gdb \
    cmake \
    zsh \
    fish \
    tmux \
    python3 \
    python3-pip \
    curl \
    ca-certificates && \
    curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*