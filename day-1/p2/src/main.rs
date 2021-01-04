use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{Instant};
    

fn main(){
        let path = Path::new("../input.txt");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
    
        let mut input_string = String::new();
        match file.read_to_string(&mut input_string) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }
        let start = Instant::now();
        let mut total = 0;
        // print!("{}", input_string);
        let mut seen = false;
        for (i, c) in input_string.chars().enumerate() {
            if c == '('   {
                total += 1;
            }
            if c == ')'{
                total -= 1;
            }

            if total == -1 && seen == false{
                print!("\n Part2: {}", i + 1);
                seen = true;
            }
        }
        print!("\n Part1: {}",total);
        let end = start.elapsed().as_micros();
        print!("\n execution time in microseconds {}", end);
}