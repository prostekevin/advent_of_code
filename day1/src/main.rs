use std::{env, fs::File, io::{BufReader, BufRead}};

fn get_filepath() -> String {
    env::args().skip(1).collect()
}

fn parse_data() -> Vec<i64> {
    let filepath = get_filepath();
    let file = File::open(filepath).expect("file not found");
    let buff = BufReader::new(file);
    
    let numbers: Vec<i64> = buff.lines()
                                .map(|line| line.unwrap().parse::<i64>().unwrap())
                                .collect();
    numbers
}

fn count_larger(data: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;
    data.iter().zip(data.iter().skip(1))
                .for_each(|tuple| if tuple.0 < tuple.1 { sum += 1 } );

    sum
}

fn three_measurement(data: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;

    data.windows(3).zip(data.windows(3).skip(1))
                        .for_each(|arrs| if arrs.0.iter().sum::<i64>() < arrs.1.iter().sum() { sum += 1 });

    sum
}
fn main() {
    let data = parse_data();
    let x = count_larger(&data);
    let y = three_measurement(&data);
    println!("{}", x);
    println!("{}", y);
}
