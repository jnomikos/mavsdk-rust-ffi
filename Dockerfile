FROM devraymondsh/ubuntu-rust:24.04
WORKDIR /workspace


RUN apt-get update 
RUN apt-get install -y clang libclang-dev