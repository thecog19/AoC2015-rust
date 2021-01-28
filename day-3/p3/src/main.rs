use std::time::{Instant};
use std::fs;

fn main(){
       
        let file ="../input.txt";
        let input_string: String = fs::read_to_string(file).unwrap();

        // let start = Instant::now();
  
        let mut santa_1_x = 4000;
        let mut santa_2_x = 4000;
        let mut santa_3_x = 4000;
        let mut santa_1_y = 4000;
        let mut santa_2_y = 4000;
        let mut santa_3_y = 4000;

        let mut curr_turn = 0;


        let mut visited_santa_1 = vec![0; 8000*8000];
        let mut visited_other_santas = vec![0; 8000*8000];

        visited_santa_1[santa_1_x*8000 + santa_1_y ] = 1;
        visited_other_santas[santa_2_x*8000 + santa_2_y ] = 1;

        let mut p1 = 1;
        let mut p2 = 1;
        let start = Instant::now();
        for c in input_string.chars() { 
            if(c == '^'){
                santa_1_x += 1;
                if(curr_turn % 2 == 0){
                    santa_2_x += 1;
                }else{
                    santa_3_x += 1;
                }
            }
            else if(c == 'v'){
                santa_1_x -= 1;
                if(curr_turn % 2 == 0){
                    santa_2_x -= 1
                }else{
                    santa_3_x -= 1
                }
            }
            else if(c == '>'){
                santa_1_y += 1;
                if(curr_turn % 2 == 0){
                    santa_2_y += 1
                }else{
                    santa_3_y += 1
                }
            }
            else if(c == '<'){
                santa_1_y -= 1;
                if(curr_turn % 2 == 0){
                    santa_2_y -= 1
                }else{
                    santa_3_y -= 1
                }
            }

            if visited_santa_1[santa_1_x*8000 + santa_1_y ] != 1{
                visited_santa_1[santa_1_x*8000 + santa_1_y ] = 1;
                p1 += 1;
            }
            
            if(curr_turn % 2 == 0){
                if visited_other_santas[santa_2_x*8000 + santa_2_y ] != 1{
                    visited_other_santas[santa_2_x*8000 + santa_2_y ] = 1;
                    p2 += 1;
                }
            }else{
                if visited_other_santas[santa_3_x*8000 + santa_3_y ] != 1{
                    visited_other_santas[santa_3_x*8000 + santa_3_y ] = 1;
                    p2 += 1;
                }
            }
            curr_turn += 1;
        }
        print!("\n day 1: {}", p1);
        print!("\n day 2: {}", p2);
        let end = start.elapsed().as_micros();
        print!("\n execution time in microseconds {}", end);
}