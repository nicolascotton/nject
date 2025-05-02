use nject::{Iterable, injectable, provider};
use pubsub::{FirstMessage, SecondMessage, Subscriber};

#[injectable]
#[provider]
struct Publisher(
    #[import] pubsub::first::Module,
    #[import] pubsub::second::Module,
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
    #[provider]
    struct InitProvider;

    let publisher = InitProvider.provide::<Publisher>();

    publisher
        .publish(&FirstMessage)
        .expect("Failed to publish first message");

    publisher
        .publish(&SecondMessage)
        .expect("Failed to publish second message");
}
