use nject::{injectable, module};

#[injectable]
struct Subscriber;
impl super::Subscriber<super::FirstMessage> for Subscriber {
    fn handle(&self, _msg: &super::FirstMessage) -> Result<(), String> {
        println!("2: Handling first message");
        Ok(())
    }
}
impl super::Subscriber<super::SecondMessage> for Subscriber {
    fn handle(&self, _msg: &super::SecondMessage) -> Result<(), String> {
        println!("2: Handling second message");
        Ok(())
    }
}

#[injectable]
#[module(crate::second::Self)]
#[export(&'prov dyn crate::Subscriber<crate::FirstMessage>, &self.0)]
#[export(&'prov dyn crate::Subscriber<crate::SecondMessage>, &self.0)]
pub struct Module(Subscriber);
