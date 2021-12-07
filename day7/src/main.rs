

fn answer<F>(data: &str, fuel_burn: F) -> i32
where
    F: Fn(i32) -> i32
{
    let crab_pos: Vec<i32> = data.split(',').map(|n| n.parse::<i32>().unwrap()).collect();

    let min_pos = *crab_pos.iter().min().unwrap();
    let max_pos = *crab_pos.iter().max().unwrap();

    let min_fuel = (min_pos..=max_pos).map(|target_pos| {
        crab_pos.iter().fold(0, |acc, &curr_pos| {
            acc + fuel_burn((curr_pos - target_pos).abs())
        })
    }).min().unwrap();
    min_fuel
}

fn main() {
    let data = include_str!("input.txt");
    println!("{}", answer(data, |s| s));
    println!("{}", answer(data,|s| s * (s + 1) /2 ));
}
