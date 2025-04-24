pub mod first;
pub mod second;

pub struct FirstMessage;
pub struct SecondMessage;

pub trait Subscriber<Message> {
    fn handle(&self, msg: &Message) -> Result<(), String>;
}
