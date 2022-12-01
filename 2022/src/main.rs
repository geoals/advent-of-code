use std::{env::args, fs::read_to_string};

mod day1;

fn main() {
    day1::part_one("\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
");
    // let day = args().nth(1).unwrap();

    // let path = format!("input/input_{}.txt", day);
    // println!("{}", path);
    // let input = read_to_string(path).unwrap();

    // match day.as_str() {
    //     "1" => {
    //         println!("Part one: {}", day1::part_one(&input));
    //         // println!("Part two: {}", day1::part_two(&input));
    //     }
    //     _ => (),
    // }
}
