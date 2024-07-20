use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

use futures::{future::BoxFuture, Future, Stream};

#[derive(Debug, Default)]
struct SharedState {
    should_tick: bool,
    total: u64,
    waker: Option<Waker>,
}

pub struct Scheduler {
    delay: Duration,
    callback: Box<dyn Fn(u64) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    shared_state: Arc<Mutex<SharedState>>,
}

impl Scheduler {
    pub fn new(
        delay: Duration,
        cb: impl Fn(u64) -> Pin<Box<dyn Future<Output = ()> + Send>> + 'static,
    ) -> Self {
        let state = Arc::new(Mutex::new(SharedState::default()));

        let thread_shared_state = state.clone();
        thread::spawn(move || loop {
            thread::sleep(delay);
            let mut shared_state = thread_shared_state.lock().expect("lock poisoned");

            shared_state.should_tick = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        Self {
            delay,
            callback: Box::new(cb),
            shared_state: state,
        }
    }
}

impl Stream for Scheduler {
    type Item = BoxFuture<'static, ()>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut state = self.shared_state.lock().expect("lock not poisoned");
        match state.should_tick {
            false => {
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            true => {
                state.total = state.total + self.delay.as_secs();
                state.should_tick = false;
                Poll::Ready(Some((self.callback)(state.total)))
            }
        }
    }
}
