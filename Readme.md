# Introducción

Esta aplicación es solo una prueba de concepto, con el fin de evaluar la confiabilidad de los paquetes disponibles en Rust 
para el uso del broker de mensajería Kafka.

En este proyecto probamos la capacidad para consumir mensajes de un broker kafka.

# Logs con Grafana Loki

En este caso usaremos el paquete de rust log4rs para guardar el log en un archvo y a promtail para enviar estos archivosa Grafana Loki.

Para log4rs se configura desde el archivo main dentro del método setup_log.

Para promtail lo debemos configurar como un nuevo servicio y el archivo de configuración esta en .devops/docker/develop/promtail_config.yml.
La carpeta en la que esta el log de la aplicación se monta como volumen en promtail para que los archivos se comuniquen de forma correcta.


# Docker

Para levantar la aplicación en entorno de desarrollo ejecutar:

```bash
#Levantar contenedor
docker-compose -f .devops/docker/develop/docker-compose.yml up

#Ingresar a contenedor
docker exec -it kafka-rust-consumer bash

#Ejecutar app
docker exec -it kafka-rust-consumer cargo run
docker exec -it kafka-rust-consumer cargo run --release

```

# Compilar

Generar ejecutable:

```bash
docker exec -it kafka-rust-consumer cargo build --release
docker exec -it kafka-rust-consumer  ./target/  
```

