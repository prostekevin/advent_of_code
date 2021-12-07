use std::collections::HashMap;


type Fraction = (i32, i32);

struct VLines {
    from: Fraction,
    to: Fraction,
}

fn string_to_frac(s: &str) -> Fraction {
    let (x_s, y_s) = s.split_once(',').unwrap();
    (x_s.parse::<i32>().unwrap(), y_s.parse::<i32>().unwrap())
}

fn range_between_nums(start: i32, end: i32) -> Box<dyn Iterator<Item = i32>> {
    if start < end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    }
}

fn answer(data: &str, do_diagonal: bool) {
    let mut num_of_lines = HashMap::new();
    
    data.lines().map(VLines::from_string)
                .flat_map(|vline| vline.get_all_fracs_on_line(do_diagonal))
                .for_each(|frac| {
                    num_of_lines.entry(frac)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);

                });
    
    let overlap = num_of_lines.iter().filter(|(_, line_count)| **line_count >= 2).count();

    println!("Overlap count {}", overlap);
}

impl VLines {
    fn from_string(line: &str) -> Self {
        let (from, to) = line.split_once(" -> ").unwrap();
        VLines {
            from: string_to_frac(from), 
            to: string_to_frac(to)
        }
    }
    fn get_all_fracs_on_line(&self, diagonal: bool) -> Vec<Fraction> {
        let &Self {
            from: (x1, y1),
            to: (x2, y2),
        } = self;
        if x1 == x2 {
            range_between_nums(y1, y2).map(|n| (x1, n)).collect()
        } else if y1 == y2 {
            range_between_nums(x1,x2).map(|n| (n,y1)).collect()
        } else if diagonal {
            range_between_nums(x1,x2).zip(range_between_nums(y1,y2)).collect()
        } else {
            vec![]
        }
    }
}








fn main() {
    let data = include_str!("input.txt");
    answer(data, false);
    answer(data, true);
}
