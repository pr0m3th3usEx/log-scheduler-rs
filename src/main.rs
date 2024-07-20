use std::time::Duration;

use scheduler::Scheduler;
use smol::stream::StreamExt;

mod scheduler;

async fn run(duration: Duration) {
    println!("Schedule duration: {:?}\nRunning...", duration);
    // Demonstrating how to store an asynchronous closure to a struct and use it in asynchronous context
    // The result is awaited after being received from the Stream
    let mut scheduler = Scheduler::new(duration, |time_elapsed| {
        Box::pin(async move {
            println!("New tick: {}s", time_elapsed);
        })
    });

    while let Some(fut) = scheduler.next().await {
        fut.await;
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let snd_arg = args.next().expect("Should have one argument");

    match snd_arg.parse::<u64>() {
        Ok(seconds) => smol::block_on(run(Duration::new(seconds, 0))),
        Err(err) => panic!("{}", err),
    }
}
