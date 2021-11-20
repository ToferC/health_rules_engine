use std::str::FromStr;
use std::sync::Mutex;

use async_graphql::*;
use futures::{Stream, StreamExt};
use rdkafka::{producer::FutureProducer, Message};

use crate::common_utils::Role;
use crate::get_connection_from_context;
use crate::kafka;
use crate::kafka::create_consumer;
use crate::kafka::get_kafka_consumer_group;
use crate::models::Trip;

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn latest_trip<'ctx>(
        &self, 
        ctx: &'ctx Context<'_>,
    ) -> impl Stream<Item = String> + 'ctx {
        let kafka_consumer_counter = ctx 
            .data::<Mutex<i32>>()
            .expect("Can't get Kafka consumer counter");

        let consumer_group_id = get_kafka_consumer_group(kafka_consumer_counter);
        let consumer = create_consumer(consumer_group_id);

        async_stream::stream! {
            let mut stream = consumer.stream();

            while let Some(value) = stream.next().await {
                yield match value {
                    Ok(message) => {
                        let payload = message.payload().expect("Kafka message shoudl contain payload");
                        let message = String::from_utf8_lossy(payload).to_string();
                        serde_json::from_str(&message).expect("Can't deserialize Trip")
                    }
                    Err(e) => format!("Error while Kafka message processing: {}", e)
                };
            }
        }
    }
}