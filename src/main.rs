use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::str;


/// This program demonstrates consuming messages through a `Consumer`.
/// This is a convenient client that will fit most use cases.  Note
/// that messages must be marked and committed as consumed to ensure
/// only once delivery.
fn main() {
    env_logger::init();

    let broker = "haproxy:9095".to_owned();
    let topic = "topic-test".to_owned();
    let group = "rust-group".to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {:?}", e);
    }
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
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;//Si no se ejecuta esta línea el mensaje no se marca como consumido
    }
}