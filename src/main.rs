use std::time::Duration;

use scheduler::Scheduler;
use smol::stream::StreamExt;

mod scheduler;

async fn run(duration: Duration) {
    println!("Schedule duration: {:?}\nRunning...", duration);
    let mut scheduler = Scheduler::new(duration);

    while let Some(time_elapsed) = scheduler.next().await {
        println!("New tick: {}s", time_elapsed);
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
