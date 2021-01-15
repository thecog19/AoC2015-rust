use std::time::{Instant};
use std::io::prelude::*;
use std::fs::File;

fn file_reader(path:&str) -> Vec<u8>{
    let f = File::open(path);
    let mut buffer = Vec::new();
    let mut file:File;
    match f {
        Ok(v) => file = v,
        Err(e) => panic!("error: {:?}", e),
    }

    // read the whole file
    let res = file.read_to_end(&mut buffer);
    match res {
        Ok(_) => return buffer,
        Err(e) => panic!("error: {:?}", e),
    }
}

fn main(){
        let start = Instant::now();
        let file ="../input.txt";
        let byte_buffer = file_reader(file);

        let mut day_2 = 0;
        let mut day_1 = 0;
        let mut dimensions = vec![];
        let mut num = vec![];
        for byte in byte_buffer {
            
            if byte == 120u8 || byte == 10u8 {
                let mut final_num = 0;
                for b in num.iter(){
                    final_num = final_num * 10u8 + b;
                }
                dimensions.push(final_num);
                num = vec![];
            }else{
                num.push(byte-48);
            }
              
            if byte == 10u8 {
                dimensions.sort();
                let length = dimensions[0] as u64;
                let width = dimensions[1] as u64;
                let height = dimensions[2] as u64;

                let volume: u64 = length*width*height;
                let smallest_perimiter: u64 = length*2 + width*2;

                day_2 += volume + smallest_perimiter;
                let surface_area: u64 = (length*width*2) + (length*height*2) + (height*width*2);
                day_1 += surface_area + length*width;
                dimensions = vec![]
            }
        }
        print!("day 1: {} day 2: {}", day_1, day_2);
        let end = start.elapsed().as_micros();
        print!("\n execution time in microseconds {}", end);
}