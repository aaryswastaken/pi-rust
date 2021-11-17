use std::time::{ Duration, Instant };
use indicatif::{ ProgressBar, ProgressStyle };

fn main() {
    let digits:u32 = 9; // This means we have to go to 10^(-{digits})

    // Naive approach
    let start = Instant::now();

    let mut sum:f64 = 0.0;
    for x in 0..(10 as u64).pow(digits+1) {
        if x%2 == 0 {
            sum += 1.0/(2*x+1) as f64;
        } else {
            sum -= 1.0/(2*x+1) as f64;
        }
    }

    sum *= 4.0;
    let duration = start.elapsed();

    println!("The found approximation is : {sum:.*} ; took {dur:?}", (digits as usize), sum=sum,
             dur=duration);
}