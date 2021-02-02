use std::time::{Instant};
use std::fs;
use md5::{Md5, Digest};
use std::str;
use std::thread;

fn thread_spawner(start: u64, end: u64){
    let t = thread::spawn(|| {

    });
    return t
}

fn calc_hashes(input_string: &str, start: u64, end: u64, p1_done: bool) -> Option<u64> {

    for i in start..end {
        let hash = Md5::digest((*input_string.clone() + &i.to_string()).as_bytes());
        let mut result: Option<u64> = None;
        let a = hash.as_slice();
        if !p1_done{
            if  a[0..2] == res_arr_p1{
                if a[2] < 16 {
                    return result = Some(i);
                }
            }
        }
        
        if a[0..3] == res_arr_p2{
            if a[2] < 16 {
                return result = Some(i);
            }
        }
    }
}
fn main(){
       
        let file ="../input.txt";
        let mut input_string: String = fs::read_to_string(file).unwrap();
        let start = Instant::now();
        input_string.truncate(input_string.len() - 1); 
        let mut i: u64 = 1;
        let mut p1_done = false;
        let res_arr_p1: [u8; 2] = [00; 2]; 
        let res_arr_p2: [u8; 3] = [00; 3]; 
        while true {
           
        };
        let end = start.elapsed().as_micros();
        println!("\n execution time in microseconds {}", end);
}