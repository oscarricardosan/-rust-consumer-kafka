use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::{fs, str};

use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};


use native_tls::{Certificate, TlsConnector};
use postgres::{Client, NoTls};
use postgres_native_tls::MakeTlsConnector;


/// This program demonstrates consuming messages through a `Consumer`.
/// This is a convenient client that will fit most use cases.  Note
/// that messages must be marked and committed as consumed to ensure
/// only once delivery.
fn main() {

    setup_log();

    let broker = "kafka-haproxy:9095".to_owned();
    let topic = "compras-ejecutadas".to_owned();
    let group = "rust-group".to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {:?}", e);
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {

    let cert = fs::read("db-certificate-cockroach.crt").unwrap();//es el mismo ca.crt
    let cert = Certificate::from_pem(&cert).unwrap();

    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap();

    let connector = MakeTlsConnector::new(connector);
    let mut client= Client::connect(
        "postgresql://savne:password@cockroach-haproxy:26257/defaultdb", connector
    ).unwrap();


    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;

    loop {
        let mss = con.poll()?;
        // if mss.is_empty() {
        //     println!("No messages available right now.");
        //     return Ok(());
        // }

        for ms in mss.iter() {
            for m in ms.messages() {

                let value= format!(
                    "{}:{}@{}: {:?}",
                    ms.topic(),
                    ms.partition(),
                    m.offset,
                    str::from_utf8(m.value)
                );
                info!("recibido {}", value);

                let query_result= client.query_one("
                    INSERT INTO events (type, data)
                    VALUES ('consumer', $1)
                    returning id", &[ &value],
                ).unwrap();
            }
            let _ = con.consume_messageset(ms);
            con.commit_consumed()?;//Si no se ejecuta esta línea el mensaje no se marca como consumido
        }
    }
}

fn setup_log() {

    let level = log::LevelFilter::Info;
    let file_path = "./log/consumer.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        // Especifica como quiero que se guarde en el archivo
        .encoder(Box::new(PatternEncoder::new("{d} {l} {f} {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Info),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config);

}