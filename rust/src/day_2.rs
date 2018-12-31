pub mod day_2 {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use std::collections::HashMap;

    pub fn day_2() {
        let mut contents = String::new();
        let path = env::current_dir().unwrap();
        let mut file = File::open(format!("{}/res/day_2.txt", path.display())).unwrap();
        file.read_to_string(&mut contents).unwrap();

        let split_string: Vec<&str> = contents.split("\n").collect();

        let checksum = calculat_checksum(&split_string);
        let common_letters_correct_boxes = find_common_letters(&split_string);

        println!("Day 2: {} with common letters: {}", checksum, common_letters_correct_boxes);
    }

    fn calculat_checksum(split_string: &Vec<&str>) -> i64 {
        let mut has_two_same = 0;
        let mut has_three_same = 0;
        let mut i = 0;
        for value in split_string {
            let letter_count = get_letter_count(value);
            if has_x_same(&letter_count, 2) {
                has_two_same += 1;
            }
            if has_x_same(&letter_count, 3) {
                has_three_same += 1;
            }
            i = i + 1;
        }

        has_two_same * has_three_same
    }    

    fn get_letter_count(value: &str) -> HashMap<char, u8> {
        let mut letter_count = HashMap::new();
        for letter in value.chars() {
            if letter_count.contains_key(&letter) {
                *letter_count.get_mut(&letter).unwrap() += 1;
            } else {
                letter_count.insert(letter, 1);
            }
        }
        letter_count        
    }

    fn has_x_same(letter_count: &HashMap<char, u8>, x: u8) -> bool {
        for value in letter_count.values() {
            if value == &x {
                return true;
            }
        }
        false
    }

    fn find_common_letters(boxes: &Vec<&str>) -> String {

        for (index, value) in boxes.iter().enumerate() {
            for next_index in index+1..boxes.len() {
                if are_correct_boxes(value, boxes[next_index]) {
                    return common_letters(value, boxes[next_index])
                }
            }
        }

        panic!("No correct words found!");
    }

    fn are_correct_boxes(word: &str, other: &str) -> bool {
        let mut differ = false;
        let mut iter = word.trim().chars();
        let mut other_iter = other.trim().chars();

        let mut chr = iter.next();
        while chr.is_some() {
            if chr.unwrap() != other_iter.next().unwrap() {
                
                if differ == true {
                    return false;
                } else {
                    differ = true;
                }
            }
            chr = iter.next();
        }
        differ
    }

    fn common_letters(word: &str, other: &str) -> String {
        let mut result = String::from("");
        let mut iter = word.chars();
        let mut other_iter = other.chars();

        let mut chr = iter.next();
        while chr.is_some() {
            if chr.unwrap() == other_iter.next().unwrap() {
                result.push(chr.unwrap());
            }
            chr = iter.next();
        }

        result
    }
}
