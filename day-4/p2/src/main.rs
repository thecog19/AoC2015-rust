use std::time::{Instant};
use std::fs;
use md5::{Md5, Digest};
use std::str;
use std::thread;
use std::thread::JoinHandle;

fn thread_spawner(input_string: String, start: u64, end: u64, p1_done: bool) -> JoinHandle<(Option<u64>,Option<u64>)>{
    let t = thread::spawn(move || {
		return calc_hashes(input_string, start, end, p1_done)
    });
    return t
}

fn calc_hashes(input_string: String, start: u64, end: u64, p1_done: bool) -> (Option<u64>,Option<u64>) {
    let res_arr_p1: [u8; 2] = [00; 2]; 
    let res_arr_p2: [u8; 3] = [00; 3];
    let mut result1: Option<u64> = None;
    let mut result2: Option<u64> = None;
    for i in start..end {
        let hash = Md5::digest((input_string.clone().to_string() + &i.to_string()).as_bytes());
        let a = hash.as_slice();
        if !p1_done{
            if  a[0..2] == res_arr_p1{
                if a[2] < 16 {
                        result1 = Some(i);
                        if a[2] == 0{
                            result2 = Some(i);
                            break;
                    } 
                }
            }
        }
    }
    return (result1, result2);
}
fn main(){
       
        let file ="../input.txt";
        let mut input_string: String = fs::read_to_string(file).unwrap();
        let start = Instant::now();
        input_string.truncate(input_string.len() - 1); 
        let mut i: u64 = 0;
        let mut p1_done = false;
        let chunk_size = 100_000;
        let mut offset = 0;
        let mut answer1: u64 = 0;
        let mut answer2: u64 = 0;
		'outer: loop { 
			// println!("Current offset: {}; current answer: {}",offset, answer);
            let mut children = vec![];
            while i < 16 {
                let t = thread_spawner(input_string.to_string(), i*chunk_size+offset, (i+1)*chunk_size+offset, p1_done);
                children.push(t);
                i += 1;
            }

            for child in children {
                let curr = child.join();
                match curr {
                    Ok(options) => {
                        //https://doc.rust-lang.org/reference/expressions/tuple-expr.html
                		match options.0 {
                	        Some(result) =>  {
                                            if result < answer1 || answer1 == 0 {
                                                answer1 = result;
                                            }
                            }
                			None => continue
                        };
                        match options.1 {
                	        Some(result) =>  {
                                            if result < answer2 || answer2 == 0 {
                                                answer2 = result;
                                            }
                            }
                			None => continue
                        };
                    }
                	Err(option) => panic!()
                }
            }
            
            if answer2 == 0 {
		        offset += i*chunk_size;
		        i = 0;
            } else {
            	break;
            }
        }
        let end = start.elapsed().as_micros();
        println!("Part 1: {}",answer1);
        println!("Part 2: {}",answer2);
        println!("\n execution time in microseconds {}", end);
}
