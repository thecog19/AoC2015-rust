use std::time::{Instant};
use std::fs;
use std::collections::HashMap;

fn main(){
       
        let file ="../input.txt";
        let input_string: String = fs::read_to_string(file).unwrap();

        let start = Instant::now();
  
        let mut santa_1_x = 0;
        let mut santa_2_x = 0;
        let mut santa_3_x = 0;
        let mut santa_1_y = 0;
        let mut santa_2_y = 0;
        let mut santa_3_y = 0;

        let mut curr_turn = 0;

        let mut visited_santa_1 = HashMap::new();
        let mut visited_other_santas = HashMap::new();

        visited_santa_1.insert((0,0), true);
        visited_other_santas.insert((0,0), true);

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
            
            visited_santa_1.insert((santa_1_x, santa_1_y), true);
            if(curr_turn % 2 == 0){
                visited_other_santas.entry((santa_2_x, santa_2_y)).or_insert(true);
            }else{
                visited_other_santas.entry((santa_3_x, santa_3_y)).or_insert(true);
            }
            curr_turn += 1;
        }

        let p1 = visited_santa_1.keys().len();
        let p2 = visited_other_santas.keys().len();
        print!("\n day 1: {}", p1);
        print!("\n day 2: {}", p2);
        let end = start.elapsed().as_micros();
        print!("\n execution time in microseconds {}", end);
}