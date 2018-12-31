extern crate actix;

use actix::prelude::*;
use std::time::Duration;

struct Worker;

impl Actor for Worker {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Worker just polled for the first time");
        println!("Stoping in 2 seconds");

        ctx.run_later(Duration::from_secs(2), |_, ctx| ctx.stop());
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Worker is stopping");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Worker stopped");
        println!();
        println!("Stopping system");

        System::current().stop();
    }
}

fn main() {
    let system = System::new("Life cycle");
    let _worker_addr = Worker.start();

    system.run();
}
