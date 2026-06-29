use nject::{Iterable, init, injectable, provider};
use pubsub::{FirstMessage, SecondMessage, Subscriber};

#[injectable]
#[provider]
struct Publisher(
    #[import] pubsub::first::FirstModule,
    #[import] pubsub::second::SecondModule,
);

impl Publisher {
    fn publish<'prov, Message>(&'prov self, msg: &'prov Message) -> Result<(), String>
    where
        Self: Iterable<'prov, &'prov dyn Subscriber<Message>>,
    {
        for sub in self.iter() {
            sub.handle(msg)?;
        }
        Ok(())
    }
}

fn main() {
    let publisher: Publisher = init!();

    publisher
        .publish(&FirstMessage)
        .expect("Failed to publish first message");

    publisher
        .publish(&SecondMessage)
        .expect("Failed to publish second message");
}
