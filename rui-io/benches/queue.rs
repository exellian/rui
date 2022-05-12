use rui_util::alloc::spmc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    static THREADS: usize = 10;
    static ITER: usize = 100_000_000;

    let (mut s, r) = channel(1024);

    let mut threads = vec![];

    struct State {
        average: f64,
        n: f64,
        n_message: usize,
    }
    let state = Arc::new(Mutex::new(State {
        average: 0.0,
        n: 0.0,
        n_message: 0,
    }));

    for _ in 0..THREADS {
        let recv = r.clone();
        let s = state.clone();
        threads.push(thread::spawn(move || loop {
            let t = Instant::now();
            let res = recv.try_recv();
            let elapsed = t.elapsed();
            {
                let mut guard = s.lock().unwrap();
                let x = elapsed.as_nanos() as f64;
                let n = guard.n + 1.0;
                guard.average = ((n - 1.0) * guard.average + x) / n;
                if let Some(_) = res {
                    guard.n_message += 1;
                }
            }
        }))
    }

    for _ in 0..ITER {
        s.send(0);
    }
    loop {
        {
            let guard = state.lock().unwrap();
            //println!("{}", guard.n_message);
            if guard.n_message == ITER {
                break;
            }
        }
    }
    {
        let mut guard = state.lock().unwrap();
        println!("Average exec time: {}ns", guard.average);
    }
}
