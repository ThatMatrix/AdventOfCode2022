//use std::env;
use std::fs;

fn main() {
    let file_path = String::from("./input.txt");

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut max_cal = 0;
    let mut vice_max = 0;
    let mut third_max = 0;
    let mut cur_cal = 0;

    for line in lines
    {
        if line.chars().count() < 1
        {
            if cur_cal > max_cal
            {
                third_max = vice_max;
                vice_max= max_cal;
                max_cal = cur_cal;
            }
            else if cur_cal > vice_max
            {
                third_max = vice_max;
                vice_max = cur_cal;
            }
            else if cur_cal > third_max
            {
                third_max = cur_cal;
            }

            cur_cal = 0;
            continue;
        }

        let line:i32 = line.parse()
            .expect("main: Could not parse, not give a number");

        cur_cal += line;
    }
    
    println!("The maxCal was {}", max_cal);
    println!("The sum of the top three was {}", max_cal + vice_max + third_max);
}
