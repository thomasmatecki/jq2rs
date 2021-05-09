FROM immunant/c2rust:debian-stretch-latest
RUN apt-get update && apt-get install -y autoconf
RUN cargo +nightly-2019-12-05 install c2rust --locked --root /home/docker/.cargo
RUN c2rust -V
RUN pip install scan-build --disable-pip-version-check 
# Build jq, create compile_commands.json
COPY jq /home/docker/jq
WORKDIR /home/docker/jq
RUN autoreconf -i 
RUN ./configure --with-oniguruma=builtin --disable-maintainer-mode --disable-valgrind --enable-all-static
RUN intercept-build make -j8
