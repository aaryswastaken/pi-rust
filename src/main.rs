use std::time::{ Duration, Instant };
use std::thread::{ spawn };
use std::sync::mpsc::{channel, Sender};

#[allow(dead_code)]
fn naive(digits:u32) -> (f64, Duration) {
    // Naive approach
    let start = Instant::now();

    let mut sum:f64 = 0.0;
    let mut x:u64 = 1;
    let mut y:bool = true;
    for _ in 0..(10 as u64).pow(digits+1) {
        if y {
            sum += 1.0/x as f64;
        } else {
            sum -= 1.0/x as f64;
        }
        x+=2; y=!y;
    }

    sum *= 4.0;
    let duration:Duration = start.elapsed();

    return (sum, duration);
}

fn threads(digits:u32) -> (f64, Duration) {
    let mut threads = Vec::new();
    let mut channels = Vec::new();
    let n_threads:u64 = 8; // HAS TO BE *2 TO HAVE FULL POWER
    let mult: u64 = 2 * n_threads;
    let c_max:u64 = (10 as u64).pow(digits + 1) / n_threads;

    let start = Instant::now();

    for x in 0..n_threads {
        let (tx, rx) = channel();

        channels.push(rx);

        if n_threads % 2 == 0 {
            threads.push(spawn(move || {
                let i: u64 = x; // i is the thread ID
                let tx: Sender<f64> = tx;

                let mut s: f64 = 0.0;
                let mut denominator: u64 = (2 * i + 3) as u64;

                for _y in 0..c_max {
                    s += 1.0 / denominator as f64;
                    denominator += mult;
                }

                println!("Thread {} finished with {}", i, s);
                tx.send(s.to_owned()).unwrap();
            }));
        } else {
            threads.push(spawn(move || {
                println!("Thread with id {} started", x);
                let i: u64 = x; // i is the thread ID
                let tx: Sender<f64> = tx;

                let mut s: f64 = 0.0;
                let mut denominator: u64 = (2 * i + 1) as u64;

                for _y in 0..c_max {
                    if _y % 2 == 0 {
                        s += 1.0 / denominator as f64;
                    } else {
                        s -= 1.0 / denominator as f64;
                    }
                    denominator += mult;
                }

                println!("Thread {} finished with {}", i, s);
                tx.send(s.to_owned()).unwrap();
            }));
        }
    }

    threads.into_iter().for_each(|t| {
        t.join().unwrap();
    });

    let mut s:f64 = 1.0;
    let mut invert:bool = true;
    channels.into_iter().for_each(|c| {
        if n_threads % 2 == 0 {
            if !invert {
                s += c.recv().unwrap();
            } else {
                s -= c.recv().unwrap();
            }
            invert = !invert;
        } else {
            s += c.recv().unwrap();
        }

        println!("Received from unknown thread. s is now {}", s);
    });

    s *= 4.0; // We calculated pi/4 so now we have pi

    let duration:Duration = start.elapsed();

    return (s, duration);
}

fn main() {
    let digits:u32 = 9; // This means we have to go to 10^(-{digits})

    let (sum, duration) = threads(digits);

    println!("The found approximation to the {}° digit is : {sum:.*} ; took {dur:?}", digits, (digits as usize), sum=sum,
             dur=duration);
}