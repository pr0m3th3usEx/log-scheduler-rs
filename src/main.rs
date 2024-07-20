use std::time::Duration;

async fn run(duration: Duration) {
    println!("Schedule duration: {:?}", duration);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let snd_arg = args.next().expect("Should have one argument");

    match snd_arg.parse::<u64>() {
        Ok(seconds) => smol::block_on(run(Duration::new(seconds, 0))),
        Err(err) => panic!("{}", err),
    }
}
