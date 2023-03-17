use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let n = arr.len();
    let mut pos_count = 0.0;
    let mut neg_count = 0.0;
    let mut zero_count = 0.0;

    for i in arr {
        if *i > 0 {
            pos_count += 1.0;
        } else if *i < 0 {
            neg_count += 1.0;
        } else {
            zero_count += 1.0;
        }
    }

    println!("{:.6}", pos_count / n as f64);
    println!("{:.6}", neg_count / n as f64);
    println!("{:.6}", zero_count / n as f64);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
