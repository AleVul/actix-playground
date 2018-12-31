mod worker;

use self::worker::Worker;
use actix::msgs::StartActor;
use actix::{Arbiter, System};
use futures::Future;

fn main() {
    let system = System::new("Arbiter playground");
    let arbiter_addr = Arbiter::new("Worker arbiter");

    Arbiter::spawn(
        arbiter_addr
            .send(StartActor::new(|_ctx| {
                println!("Send start actor message to arbiter");
                Worker
            }))
            .then(|_wrk_addr| {
                println!("Got addr of the worker");

                futures::future::ok::<_, ()>(())
            }),
    );

    println!("After arbiter spawned");

    system.run();
}
