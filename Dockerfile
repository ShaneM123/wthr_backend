FROM rust:latest
MAINTAINER shane moloney
WORKDIR /var/stuff/app
COPY . var/stuff/app
RUN apt-get update
RUN apt-get install -y curl apt-utils
RUN apt-get update
RUN apt-get install build-essential
RUN cd var/stuff/app/ && cargo build
RUN cd var/stuff/app/ && chmod +x startserver.sh
EXPOSE 8445
CMD cd var/stuff/app/ && ./startserver.sh
