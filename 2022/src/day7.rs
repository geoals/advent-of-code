pub fn part_one(input: &str) -> usize {
    let mut root = Directory { 
        name: "/".to_string(), 
        sub_directories: vec![], 
        files: vec![] 
    };
    // let mut current_dir = &mut root;
    let mut dir_history: Vec<&mut Directory> = vec![];

    for line in input.lines() {
        println!("{}", line);
        let mut parts = line.split_whitespace();
        let first_part = parts.next().unwrap();
        if first_part == "$" {
            let command = parts.next().unwrap();
            match command {
                "cd" => {
                    let next_dir_name = parts.next().unwrap();
                    if next_dir_name == "/" {
                        // dir_history.push(&mut root);
                        // current_dir = &mut root;
                    } else if next_dir_name == ".." {
                        dir_history.pop();
                    // } else {
                    //     let next_dir = dir_history.last().unwrap()
                    //         .sub_directories.iter_mut().find(|dir| {
                    //             dir.name == next_dir_name
                    //         }).unwrap();
                    //     dir_history.push(next_dir);
                    }
                },
                _ => continue
                // "ls" => {

                // },
                // _ => panic!()
            }

        } else if first_part == "dir" {
            // continue
            let last_dir = dir_history.last().unwrap();
            // last_dir.add_directory(parts.next().unwrap().to_string());
        // } else {
        //     dir_history.last().unwrap().add_file(
        //         first_part.parse::<usize>().unwrap(),
        //          parts.next().unwrap().to_string()
        //     );
        }
    }
    1
}

pub fn part_two(input: &str) -> usize {
    1
}

struct Directory {
    name: String,
    sub_directories: Vec<Directory>, 
    files: Vec<File>,
}

struct File {
    name: String,
    size: usize,
}

// `$ cd /` not implemented

impl Directory {
    fn add_file(&mut self, size: usize, name: String) {
        self.files.push(File { name, size });
    }

    fn add_directory(&mut self, name: String) {
        self.sub_directories.push(Directory::new(name));
    }

    fn new(name: String) -> Self {
        Directory { name, sub_directories: vec![], files: vec![] }
    }

    fn size() {

    }
}

#[test]
fn example_input_part1() {
    let input = include_str!("../input/sample_7.txt");
    assert_eq!(part_one(input), 95437);
}