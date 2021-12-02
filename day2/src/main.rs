use std::{env, fs::File, io::{BufRead, BufReader}};

fn get_filepath() -> String {
    env::args().skip(1).collect()
}

fn parse_data() -> Vec<(String, i32)> {
    let filepath = get_filepath();
    let file = File::open(filepath).expect("file not found");
    let buffer = BufReader::new(file);

    let commands: Vec<_> = buffer.lines().map(|str| str.unwrap()).collect();

    let data: Vec<(String, i32)>= commands.iter().map(move |x| x.split_at(x.find(" ").unwrap())).map(|f| (f.0.to_owned(), f.1[1..].parse::<i32>().unwrap())).collect();

    data
}   


// return (horizontal, vertical)
fn get_position(data: &Vec<(String, i32)>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;

    for (s, x ) in data.iter() {
        match  (s.as_str(), x) {
            ("forward", x ) =>      horizontal += x,
            ("down", x ) => depth += x,
            ("up", x ) => depth -= x,
            (_, _) => ()
        }
    }
    (horizontal,depth)
}

fn get_position_with_aim(data: &Vec<(String, i32)>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (s, x ) in data.iter() {
        match  (s.as_str(), x) {
            ("forward", x ) => {
                    horizontal += x;
                    depth += aim*x;
            },
            ("down", x ) => aim += x,
            ("up", x ) => aim -= x,
            (_, _) => ()
        }
    }
    (horizontal,depth)
}
fn main() {
    let data = parse_data();
    let (hor, ver) = get_position(&data);
    let (ac_hor, ac_ver) = get_position_with_aim(&data);
    println!("1. multiply {}", hor * ver);
    println!("2. multiply {}", ac_hor * ac_ver);
}
