use actix::{Actor, Context, Message, Handler, Arbiter};
use std::time::Duration;

pub struct Worker;

pub struct Ping(pub u32);

impl Actor for Worker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Worker polled");
        println!("Running in arbiter: {}", Arbiter::name());
    }
}

impl Message for Ping {
    type Result = u32;
}

impl Handler<Ping> for Worker {
    type Result = u32;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        std::thread::sleep(Duration::from_secs(25));
        msg.0
    }
}