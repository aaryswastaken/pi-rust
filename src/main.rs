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
    // Threaded approach

    // Define the thread-inherent vectors
    let mut threads = Vec::new();
    let mut channels = Vec::new();

    // define some constants
    let n_threads:u64 = 8; // HAS TO BE *2 TO HAVE FULL POWER
    let mult: u64 = 2 * n_threads;
    let c_max:u64 = (10 as u64).pow(digits + 1) / n_threads;

    let start = Instant::now(); // Start the clock

    for x in 0..n_threads { // For every thread we are supposed to launch
        let (tx, rx) = channel(); // Create a channel

        channels.push(rx); // Append the rx channel so that we can retrieve the value

        if n_threads % 2 == 0 { // Not compact but wayyy faster
            // If our n_threads is a factor of two we can just calculate every sum and do the +/-
            // at the end which will be faster (no if -> faster)

            // Create new thread
            threads.push(spawn(move || {
                let i: u64 = x; // i is the thread ID
                let tx: Sender<f64> = tx; // Assign the tx channel

                let mut s: f64 = 0.0;
                let mut denominator: u64 = (2 * i + 3) as u64;

                for _y in 0..c_max {
                    s += 1.0 / denominator as f64;
                    denominator += mult;
                }

                println!("Thread {} finished with {}", i, s);
                tx.send(s.to_owned()).unwrap(); // Send the little sum
            }));
        } else {
            // Same as above but as the n_threads % 2 == 1 the little sum has to sum subsequent
            // different polarities and we can't do it in post

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

    // Wait each thread to finish
    threads.into_iter().for_each(|t| {
        t.join().unwrap();
    });

    // Start the final sum process
    let mut s:f64 = 1.0;
    let mut invert:bool = true;
    channels.into_iter().for_each(|c| {
        if n_threads % 2 == 0 {
            // If we didn't do the alternative sum/substr we do it now
            if !invert {
                s += c.recv().unwrap();
            } else {
                s -= c.recv().unwrap();
            }

            invert = !invert;
        } else {
            s += c.recv().unwrap();
        }

        // println!("Received from unknown thread. s is now {}", s); // Do some log
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