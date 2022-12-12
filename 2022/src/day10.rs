pub fn part_one(input: &str) -> i32 {
    let cycles = get_x_values(input);

    (20..221).step_by(40).map(|cycle| {
        cycles[cycle - 1] * cycle as i32
    }).sum()
}

pub fn part_two(input: &str) {
    let cycles = get_x_values(input);

    for y in 0..6 {
        for x in 0..40 {
            let cycle = (x + 1 + y * 40 - 1) as usize;
            let visible = x <= cycles[cycle] + 1 && x >= cycles[cycle] - 1;
            let pixel = if visible { "#" } else { "." };
            print!("{}", pixel);
        }
        println!();
    }
}

fn get_x_values(input: &str) -> Vec<i32> {
    let mut x = 1;
    let mut cycles: Vec<i32> = vec![x];
    for line in input.lines() {
        if line == "noop" {
            cycles.push(x);
        } else {
            let number: i32 = line.split_once(' ').unwrap().1.parse().unwrap();
            for i in 0..2 {
                if i == 1 {
                    x += number;
                } 
                cycles.push(x);
            }
        }
    }
    cycles
}

#[test]
fn example_input_part1_2() {
    let input = include_str!("../input/sample_10.txt");
    assert_eq!(part_one(input), 13140);
}