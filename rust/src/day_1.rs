pub mod day_1 {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn day_1() {
        let path = env::current_dir().unwrap();
        let mut file = File::open(format!("{}/res/day_1.txt", path.display())).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let split_string: Vec<&str> = contents.split("\n").collect();

        let mut result = 0;
        let mut i = 0;
        for value in split_string {
            result += read_value(&value);
            i = i + 1;
        }

        println!("Day 1: {}", result);
    }

    fn read_value(value: &str) -> i32 {
        if value.starts_with("+") {
            let split_value: Vec<&str> = value.split("+").collect();
            return split_value[1].trim().parse::<i32>().unwrap();
        } else if value.starts_with("-") {
            let split_value: Vec<&str> = value.split("-").collect();
            return -split_value[1].trim().parse::<i32>().unwrap();
        } else {
            panic!("This shouldn't happen, could not be read value: {}", value);
        }
    }
}
