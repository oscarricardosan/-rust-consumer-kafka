FROM debian:buster-slim

ENV TZ="America/Bogota"

RUN apt update
RUN apt install build-essential zlib1g-dev wget -y

#necesaria para usar rkafka
RUN apt clean && apt update
RUN apt install -y build-essential libssl-dev libsasl2-dev
RUN apt install -y cmake


RUN apt clean && apt update
RUN apt install curl -y --fix-missing

#Instalación de paquetes base para linux dev
RUN apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev -y
RUN apt install libpq-dev -y #para usar el ORM Diesel
RUN apt install pkg-config -y


#instalacion de RUST
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

#RUN rustup install 1.60.0

RUN rm -rf /var/lib/apt/lists/*

WORKDIR /home/app

ENTRYPOINT ["tail", "-f", "/dev/null"]
