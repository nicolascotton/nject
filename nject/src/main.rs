use nject::{injectable, provider, Injectable, Provider};

trait Tone {
    fn test(&self) -> String;
}

#[injectable]
struct Two {}

struct One {
    two: Two,
    tone: Box<dyn Tone>,
}

fn main() {
    let prov = Prov {};
    let toto: Toto = prov.provide();
    //let tata = inject::<One, One>();
}

#[injectable]
struct Toto {
    two: Two,
}

#[provider]
struct Prov;

#[injectable]
struct Toto2<T>(T);
