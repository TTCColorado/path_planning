FROM python:3.9

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH=/root/.cargo/bin:$PATH

# RUN apt-get update && apt-get install -y --no-install-recommends python3.6-dev python3.6-pip python-dev && rm -rf /var/lib/apt/lists/*

RUN pip3 install setuptools
RUN pip3 install setuptools wheel setuptools-rust

# RUN apt-get update && apt-get install -y \
#   xz-utils \
#   build-essential \
#   curl \
#   libncurses5 \
#   && rm -rf /var/lib/apt/lists/* \
#   && curl -SL https://github.com/llvm/llvm-project/releases/download/llvmorg-10.0.0/clang+llvm-10.0.0-x86_64-linux-gnu-ubuntu-18.04.tar.xz \
#   | tar -xJC . && \
#   mv clang+llvm-10.0.0-x86_64-linux-gnu-ubuntu-18.04 clang_10.0.0 && \
#   echo 'export PATH=/clang_10.0.0/bin:$PATH' >> ~/.bashrc && \
#   echo 'export LD_LIBRARY_PATH=/clang_10.0.0/lib:$LD_LIBRARY_PATH' >> ~/.bashrc

RUN apt-get update && apt-get install -y llvm-dev libclang-dev clang-9 

WORKDIR /opt/path-planning

CMD ["python3", "setup.py", "bdist_wheel"]
