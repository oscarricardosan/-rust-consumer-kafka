# Introducción

Esta aplicación es solo una prueba de concepto, con el fin de evaluar la confiabilidad de los paquetes disponibles en Rust 
para el uso del broker de mensajería Kafka.

En este proyecto probamos la capacidad para consumir mensajes de un broker kafka.

# Docker

Para levantar la aplicación en entorno de desarrollo ejecutar:

```bash
#Levantar contenedor
docker-compose -f .devops/docker/develop/docker-compose.yml up

#Ingresar a contenedor
docker exec -it kafka-rust-consumer bash

#Ejecutar app
docker exec -it kafka-rust-consumer cargo run
```

# Compilar

Generar ejecutable:

```bash
docker exec -it kafka-rust-consumer cargo build --release
docker exec -it kafka-rust-consumer  ./target/  
```

