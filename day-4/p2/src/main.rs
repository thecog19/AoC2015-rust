use std::time::{Instant};
use std::fs;
use md5::{Md5, Digest};
use std::str;
use std::thread;
use std::thread::JoinHandle;

fn thread_spawner(input_string: String, start: u64, end: u64, p1_done: bool) -> JoinHandle<Option<u64>>{
    let t = thread::spawn(move || {
		return calc_hashes(input_string, start, end, p1_done)
    });
    return t
}

fn calc_hashes(input_string: String, start: u64, end: u64, p1_done: bool) -> Option<u64> {
    let res_arr_p1: [u8; 2] = [00; 2]; 
    let res_arr_p2: [u8; 3] = [00; 3];
    let mut result = None;
    for i in start..end {
        let hash = Md5::digest((input_string.clone().to_string() + &i.to_string()).as_bytes());
        let mut result: Option<u64> = None;
        let a = hash.as_slice();
        
        let mut q = 0;
        println!("Hash {:?} = {}",i,a[0]);
        for n in 0..1_000_000 {
        	q += 1;
        };
        q = q - 1_000_000;
        
        if a[0] == 0 {
        	result = Some(i);
        	break
        }
        
//        if !p1_done{
//            if  a[0..2] == res_arr_p1{
//                if a[2] < 16 {
//                    result = Some(i);
//                    break
//                }
//            }
//        }
//        
//        if a[0..3] == res_arr_p2{
//            if a[2] < 16 {
//                result = Some(i);
//                break
//            }
//        }
    }
    return result;
}
fn main(){
       
        let file ="../Inputs/Day4Input.txt";
        let mut input_string: String = fs::read_to_string(file).unwrap();
        let start = Instant::now();
        input_string.truncate(input_string.len() - 1); 
        let mut i: u64 = 0;
        let mut p1_done = false;
        let chunk_size = 1000;
        let mut offset = 0;
        let mut answer: u64 = 0;
		loop { 
			println!("Current offset: {}; current answer: {}",offset, answer);
            let mut children = vec![];
            while i < 1 {
                //t = thread_spawner(&input_string, i*chunk_size+offset, (i+1)*chunk_size + offset, p1_done);
                let t = thread_spawner(input_string.to_string(), i*chunk_size+offset, (i+1)*chunk_size+offset, p1_done);
                children.push(t);
                i += 1;
            }

            for child in children {
                let curr = child.join();
                match curr {
                	Ok(option) => 
                		match option {
                	        Some(result) => println!("Result: {}",result),
                			None => continue
                		},
                	Err(option) => panic!()
                }
            }
            
            if answer == 0 {
		        offset += i*chunk_size;
		        i = 0;
            } else {
            	break;
            }
        }
        let end = start.elapsed().as_micros();
        println!("Part 1: {}",answer);
        println!("\n execution time in microseconds {}", end);
}
