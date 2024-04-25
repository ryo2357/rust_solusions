FROM mcr.microsoft.com/devcontainers/rust

RUN apt-get update && \
  apt-get install -y git-flow