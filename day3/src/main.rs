
fn main() {
    let file_input = include_str!("test.input");
    let m = matrix_transpose(&parse_to_matrix(file_input));
    let size = m.len();
    let mut gamma:u32 = 0;
    for i in 0..size {
        gamma *= 2;
        if m[i].iter().sum::<u32>() > (m[i].len() / 2) as u32 {
            gamma += 1;
        }

        
    }

    let eps = !gamma & ((1 << size) - 1);
    let o2 = get_o2(file_input);
    let co2 = get_co2(file_input);
    println!("{:?}",  eps* gamma);
    println!("{:?}", to_u32(&o2) * to_u32(&co2) );
}

fn to_u32(vec: &[u32]) -> u32 {
    let mut x = 0;
    for i in vec.iter(){
        x *= 2;
        if *i == 1 {
            x += 1;
        }
    }
    x
}

fn get_co2(input: &str) -> Vec<u32> {
    let mut m = parse_to_matrix(input);
    let size = m[0].len();
    for i in 0..size {
        let temp = matrix_transpose(&m);
        if temp[i].iter().filter(|n| **n == 1).count() >= temp[i].iter().filter(|n| **n == 0).count() {
            m.retain(|v | v[i] != 1);
        } else {
            m.retain(|v | v[i] != 0);
        }
        if m.len() == 1 {
            return m.pop().unwrap();
        }
    }
    let x = Vec::new();
    x
}
fn get_o2(input: &str) -> Vec<u32> {
    let mut m = parse_to_matrix(input);
    let size = m[0].len();
    for i in 0..size {
        let temp = matrix_transpose(&m);
        if temp[i].iter().filter(|n| **n == 1).count() >= temp[i].iter().filter(|n| **n == 0).count() {
            m.retain(|v | v[i] != 0);
        } else {
            m.retain(|v | v[i] != 1);
        }
        if m.len() == 1 {
            return m.pop().unwrap();
        }
    }
    let x = Vec::new();
    x
}

fn parse_to_matrix(input: &str) -> Vec<Vec<u32>> {
    let mut matrix = Vec::new();
    
    for line in input.lines() {
        let mut i = Vec::new();
        for c in line.chars() {
            i.push(c.to_digit(10).unwrap());
            
        }
        matrix.push(i);
    }
    matrix
}




fn matrix_transpose(m: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}