use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::str;

use std::process;
use std::thread::sleep;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::filter::LevelFilter;

use tracing::{info, info_span};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), tracing_loki::Error> {
    let (layer, task) = tracing_loki::builder()
        .label("host", "10.0.0.2")?
        .label("application", "rust-kafka-consumer")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://10.0.0.3:3100").unwrap())?;

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(layer)
        .with(LevelFilter::INFO)//Transmita solo los eventos de tipo info
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        .init();

    // The background task needs to be spawned so the logs actually get
    // delivered.
    tokio::spawn(task);

    let broker = "haproxy:9095".to_owned();
    let topic = "compras-ejecutadas".to_owned();
    let group = "grupo1".to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {:?}", e);
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
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
                println!(
                    "{}:{}@{}: {:?}",
                    ms.topic(),
                    ms.partition(),
                    m.offset,
                    str::from_utf8(m.value)
                );

                let message= format!("Mensaje id -{} recibido", m.offset).to_string();
                let data= str::from_utf8(m.value).unwrap().to_string();

                tracing::info!(
                    task = "rust-kafka-consumer",
                    result = "success",
                    message,
                    data = data
                );
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;//Si no se ejecuta esta línea el mensaje no se marca como consumido
    }
}