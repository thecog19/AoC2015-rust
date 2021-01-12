use std::time::{Instant};
use std::fs;
    

fn main(){
        let file ="../input.txt";
        let input_string: String = fs::read_to_string(file).unwrap();
        let lines = input_string.lines();
        let lines2 = input_string.lines();

        let start = Instant::now();
        let mut day_2 = 0;
        let mut day_1 = 0;
        for string in lines {
            let byte_dimensions = string.as_bytes();
            let mut dimensions = vec![];
            let mut num = vec![];
            
            for byte in byte_dimensions {
  
                if(*byte == 120u8){
                    let mut final_num = 0;
                    for b in num.iter(){
                        final_num = final_num * 10u8 + b;
                    }
                    dimensions.push(final_num);
                    num = vec![];
                }else{
                    num.push(*byte-48);
                }
                
            }
            let mut final_num = 0;
            for b in num.iter(){
                final_num = final_num * 10u8 + b;
            }
            dimensions.push(final_num);
            num = vec![];

            dimensions.sort();
            let length = dimensions[0] as u64;
            let width = dimensions[1] as u64;
            let height = dimensions[2] as u64;

            let volume: u64 = length*width*height;
            let smallest_perimiter: u64 = length*2 + width*2;

            day_2 += volume + smallest_perimiter;
            let surface_area: u64 = (length*width*2) + (length*height*2) + (height*width*2);
            day_1 += surface_area + length*width;
        }
        print!("day 1: {} day 2: {}", day_1, day_2);
        let end = start.elapsed().as_micros();
        print!("\n execution time in microseconds {}", end);
}