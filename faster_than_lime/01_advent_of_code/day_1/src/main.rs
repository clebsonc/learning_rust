use std::fs::read_to_string;


// problem statement: https://fasterthanli.me/series/advent-of-code-2022/part-1#the-problem-statement
//
// Each group of lines separated by an empty line is a list of food items an elf is carrying: each
// line corresponds to the number of calories in that food. So, the first elf is carrying three
// food items: one is 1000 calories, the second is 2000 calories, the third is 3000 calories. The
// second elf is carrying only one food item of 4000 calories, etc. We must answer "how many
// calories is the elf who carries the most calories carrying?"


mod characters {
    #[derive(Debug)]
    pub struct Elf {
        food: Vec<i32>,
    }

    impl Elf {
        pub fn compute_calories(&self) -> i32 {
            self.food.iter().sum::<i32>()
        }
    }

    pub struct ElfGroup {
        elves: Vec<Elf>,
    }

    impl ElfGroup {
        pub fn print_elf_calories_with_max_calorie(&self) {
            let mut max_calorie = 0;
            for elf in self.elves.iter() {
                let cal = elf.compute_calories();
                println!("Elf calorie: {}", cal);
                if cal > max_calorie {
                    max_calorie = cal;
                }
            }
            println!("\nMax calorie: {}", max_calorie);
        }
    }

    pub fn build_elf_group(values: Vec<Vec<i32>>) -> ElfGroup {
        let mut elves: Vec<Elf> = Vec::new();

        for val in values {
            let elf: Elf = Elf {
                food: val
            };
            elves.push(elf);
        }
        ElfGroup {
           elves
        }
    }
}

fn read_from_file(file_name: &str) -> Vec<Vec<i32>> {
    let parser = read_to_string(file_name);
    let data: String = match parser {
        Ok(s) => s,
        Err(e) => panic!("Error while loading the file. {}", e)
    };
    let splited_data: Vec<&str> = data.split("\n\n").collect();

    let mut parsed_data: Vec<Vec<i32>> = Vec::new();
    for val in splited_data {
        let mut parsed_values: Vec<i32> = Vec::new();
        let splited: Vec<&str> = val.trim().split("\n").collect();
        for i in splited {
            parsed_values.push(i.to_string().parse::<i32>().unwrap());
        }
        parsed_data.push(parsed_values);
    }
    parsed_data
}


fn main() {
    use crate::characters::{build_elf_group, ElfGroup};
    let full_data: Vec<Vec<i32>> = read_from_file("src/input.txt");
    let elves: ElfGroup = build_elf_group(full_data);
    elves.print_elf_calories_with_max_calorie();
}
