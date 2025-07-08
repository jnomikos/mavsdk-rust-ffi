docker run -it --rm   -v "$PWD":/workspace   -w /workspace   devraymondsh/ubuntu-rust:24.04   bash

FROM devraymondsh/ubuntu-rust:24.04
WORKDIR /workspace
COPY . /workspace


RUN apt-get update 
RUN apt-get install -y clang libclang-dev
RUN apt-get install python3-pip
