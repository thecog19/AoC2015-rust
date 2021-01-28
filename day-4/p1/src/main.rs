use std::time::{Instant};
use std::fs;
use md5::{Md5, Digest};
use std::str;

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
            let hash = Md5::digest((input_string.clone() + &i.to_string()).as_bytes());
            // println!("MD5 {}: {:x}",i,hash);
            i += 1;
           
            let a = hash.as_slice();
            if !p1_done{
                if  a[0..2] == res_arr_p1{
                    if a[2] < 16 {
                        p1_done = true;
                        println!("P1 {}: {:x}",i-1,hash);
                    }
                }
            }
            
            if a[0..3] == res_arr_p2{
                if a[2] < 16 {
                    println!("P2 {}: {:x}",i-1,hash);
                    break;
                }
            }
        };
        let end = start.elapsed().as_micros();
        println!("\n execution time in microseconds {}", end);
}