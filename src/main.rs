use num_rational::{ Ratio };
use num_bigint::{ BigInt };
use indicatif::{ ProgressBar, ProgressStyle };
use std::fs::{ File };
use bytes::Bytes;

use std::io;
use std::io::Write;

// fn next_iter(i:u64, s:Ratio<u64>, i_max:u64) -> Ratio<u64>{
//     if i == i_max {
//         return s
//     }
//     return next_iter(i+1, s+Ratio::new_raw(1, if i%2==0 {2*i+1} else {(0-2)*i+1}), i_max)
// }
//
// fn pi(n_iter:u64) -> Ratio<u64> {
//     return next_iter(1, Ratio::new_raw(1, 1), n_iter)
// }

type T = BigInt;

fn format(r: Ratio<T>, max_decimals:u64) -> Bytes {
    let mut fract = r.fract();
    for _ in 0..max_decimals {
        if fract.is_integer() {
            break; // This means we already got all digits available
        }
        // By multiplying by 10 we move the digit to the "whole part" of the ratio
        fract = fract * BigInt::from(10);
    }
    // to_integer() gives us a representation with the decimal values truncated.
    // fract contains up to max_decimals of the digits after the decimal value as
    // the whole (before the value) so printing those values will give us the post
    // decimal digits
    return Bytes::from(format!("{}.{}\n", r.to_integer(), fract.to_integer()));
}

fn pi(n_iter:u64) -> Ratio<T> {
    let one: fn() -> BigInt = || { BigInt::from(1) };

    let mut s:Ratio<T> = Ratio::new(one(), one());
    let mut n:bool = true;

    let bar:ProgressBar = ProgressBar::new(n_iter);
    // bar.set_draw_delta(n_iter / 1000);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise} - {eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {percent}%")
        .progress_chars("#>-"));

    for x in 1..n_iter {
        // println!("{}", x);
        if n {
            s = s - Ratio::new(one(), BigInt::from(x));
        } else {
            s = s + Ratio::new(one(), BigInt::from(x));
        }

        if x%100 == 0 {
            s = s.reduced();
            // log.write(format(s.reduced(), 50));
        }

        n = !n;
        bar.inc(1);
    }

    bar.finish();

    return s
}

fn print_as_decimal(r: Ratio<T>, max_decimals: i32) {
    // We get the fractional part. We want to get as many digits as possible from here.
    let mut fract = r.fract();
    for _ in 0..max_decimals {
        if fract.is_integer() {
            break; // This means we already got all digits available
        }
        // By multiplying by 10 we move the digit to the "whole part" of the ratio
        fract = fract * BigInt::from(10);
    }
    // to_integer() gives us a representation with the decimal values truncated.
    // fract contains up to max_decimals of the digits after the decimal value as
    // the whole (before the value) so printing those values will give us the post
    // decimal digits
    println!("{}.{}", r.to_integer(), fract.to_integer());
}

fn main() {
    let i:u64 = 4;
    let pi:Ratio<T> = pi((10 as u64).pow(i as u32)) * BigInt::from(10);
    println!("log(n_iter, 10) = {}", i);
    print_as_decimal(pi, 50);
}
