FROM debian:jessie
ENV DEBIAN_FRONTEND=noninteractive

EXPOSE 3000

RUN mkdir -p /usr/src
COPY target/release/ /usr/src

WORKDIR /usr/src

ENTRYPOINT ["/usr/src/main"]