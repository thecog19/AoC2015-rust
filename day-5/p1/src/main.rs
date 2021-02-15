use std::time::{Instant};
use std::fs;

fn main(){
       
        let file ="../input.txt";
        let mut input: String = fs::read_to_string(file).unwrap();
        let words = input.lines();
        let start = Instant::now();

        let mut p1: u32 = 0;
        let mut p2: u32 = 0;
        
        for word in words {
            let mut vowel_count = 0 ;
            let mut prev_char = ' ';
            let mut two_behind = ' ';
            let mut has_pair = false;
            let mut has_pair_match = false;
            let mut p1_checked = false;
            let mut p2_checked = false;
            let mut has_tryptic = false;
            let mut illegal_p1 = false;
            for c in word.chars() {
                if is_illegal(word){
                    illegal_p1 = true;
                    p1_checked = true;
                }
                if c == prev_char {
                    has_pair = true;
                }
                if is_vowel(&c){
                    vowel_count += 1;
                }
                if has_non_overlapping_pair(word, &[prev_char, c].iter().collect::<String>()){
                    has_pair_match = true
                }
                if is_tryptic([two_behind, prev_char, c].to_vec()){
                    has_tryptic = true
                }

                if vowel_count >= 3 && has_pair && !p1_checked {
                    p1 += 1;
                    p1_checked = true;
                }

                if has_tryptic && has_pair_match && !p2_checked {
                    p2 += 1;
                    p2_checked = true
                }

                if p1_checked && p2_checked {
                    break;
                }

                two_behind = prev_char;
                prev_char = c;
            }
        }
        println!("P1: {}", p1);
        println!("P2: {}", p2);
        let end = start.elapsed().as_micros();
        println!("\n execution time in microseconds {}", end);
}

fn is_vowel(c: &char) -> bool {
    return matches!(*c, 'a' | 'e' | 'i' | 'o' | 'u');
}

fn is_tryptic(arr: Vec<char>) -> bool {
    return arr[0] == arr[2]
}

fn has_non_overlapping_pair(word: &str, pair: &str) -> bool {
    return word.matches(pair).count() > 1;
}

fn is_illegal(word: &str) -> bool {
    return word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy")
}