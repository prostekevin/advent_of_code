

fn answer(data: &str, days: usize) -> i64 {
    let mut fish_t = [0;9];

    data.split(',').map(|n| n.parse::<i32>().unwrap()).for_each(|n| fish_t[n as usize] += 1);

    for _ in 0..days {
        fish_t.rotate_left(1);
        fish_t[6] += fish_t[8];
    }

    let sum: i64 = fish_t.iter().sum();
    sum
}

fn main() {
    let data = include_str!("input.txt");

    println!("First part: {}", answer(data, 80));
    println!("Second part: {}", answer(data, 256));
    
}
