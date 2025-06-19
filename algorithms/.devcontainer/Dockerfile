FROM gcc:latest

RUN apt-get update && \
    apt-get install -y build-essential gdb cmake zsh fish tmux && \
    apt-get clean && rm -rf /var/lib/apt/lists/*