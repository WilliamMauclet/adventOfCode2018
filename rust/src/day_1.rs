pub mod day_1 {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn day_1() {
        let mut file = File::open("/home/wmclt/Projects/adventOfCode2019/rust/res/input.txt")
            .expect("should work");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("should work");

        let split_string: Vec<&str> = contents.split("\n").collect();

        let mut result = 0;
        let mut i = 0;
        for value in split_string {
            // print!("{}:{}\n", i, value);
            result = result + read_value(&value);
            i = i + 1;
        }

        println!("Day 1: {}", result);
    }

    fn read_value(value: &str) -> i32 {
        if value.starts_with("+") {
            let split_value: Vec<&str> = value.split("+").collect();
            return split_value[1].trim().parse::<i32>().expect("should work");
        } else if value.starts_with("-") {
            let split_value: Vec<&str> = value.split("-").collect();
            return -split_value[1].trim().parse::<i32>().expect("should work");
        } else {
            panic!("This shouldn't happen, could not be read value: {}", value);
        }
    }
}
