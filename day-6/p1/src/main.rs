// Advent of Code Day 6 Proof of Concept
// Created April 15th, 2021.
// Last updated May 27th, 2021.

use std::fs;
use std::time::{Instant};
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Rectangle<'a> {
	xmin: i64,
	ymin: i64,
	xmax: i64,
	ymax: i64,
	light: &'a str
}

fn main() {
	let mut i = 0; 
	let array_of_value = [1,2,5,10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 1000000];
	for val in array_of_value.iter() {
		main_input(*val as usize);
	}
}


fn main_input(cutoff: usize) {
	let start = Instant::now();

	let file = "../../input3.txt";
	let input_raw: String = fs::read_to_string(file).unwrap();
	
	let mut rectangles: Vec<Rectangle> = Vec::new();
	
	let mut x_range = vec![];
	let mut y_range = vec![];
	
	for (index, line) in input_raw.lines().enumerate() {
		let mut tokens = line.split(|c| c == ' ' || c == ',');
		let mut keyword = tokens.next().unwrap();
		if keyword == "turn" {
			keyword = tokens.next().unwrap();
		};
		let xmin: i64 = tokens.next().unwrap().parse().unwrap();
		let ymin: i64 = tokens.next().unwrap().parse().unwrap();
		tokens.next();
		let xmax: i64 = tokens.next().unwrap().parse::<i64>().unwrap() + 1; // Plus one is critical!
		let ymax: i64 = tokens.next().unwrap().parse::<i64>().unwrap() + 1;
		let light: &str = keyword;
		
		x_range.push(xmin);
		x_range.push(xmax);
		y_range.push(ymin);
		y_range.push(ymax);
		
		rectangles.push(Rectangle{xmin,ymin,xmax,ymax,light});
		if index > cutoff {
			break;
		}
	}
	
	x_range.sort();
	x_range.dedup();
	y_range.sort();
	y_range.dedup();
	
	let mut x_diff = Vec::with_capacity(x_range.len());
	let mut x_hash = HashMap::new();
	
	for i in 0..(x_range.len()) {
		if i < x_range.len() - 1 {
			x_diff.push(x_range[i+1] - x_range[i]);
		} else {
			x_diff.push(1);
		}
		x_hash.insert(x_range[i],i as usize);
	};
	
	let mut y_diff = Vec::with_capacity(y_range.len());
	let mut y_hash = HashMap::new();
	
	for i in 0..(y_range.len()) {
		if i < y_range.len() - 1 {
			y_diff.push(y_range[i+1] - y_range[i]);
		} else {
			y_diff.push(1);
		}
		y_hash.insert(y_range[i],i as usize);
	};
	
	let mut first_lights = vec![vec![0i64; y_range.len()]; x_range.len()];
	let mut second_lights = vec![vec![0i64; y_range.len()]; x_range.len()];
	
	for r in rectangles {
	
		let xmin = *(x_hash.get(&r.xmin).unwrap());
		let xmax = *(x_hash.get(&r.xmax).unwrap());
		let ymin = *(y_hash.get(&r.ymin).unwrap());
		let ymax = *(y_hash.get(&r.ymax).unwrap());
	
		for row in &mut first_lights[xmin..xmax] {
			for val in &mut row[ymin..ymax] {
				*val = match r.light {
					"on" => 1,
					"off" => 0,
					"toggle" => if *val == 1 {0} else {1}, 
					_ => 0
				}
			}
		}
		
		for row in &mut second_lights[xmin..xmax] {
			for val in &mut row[ymin..ymax] {
				*val = match r.light {
					"on" => *val + 1,
					"off" => if *val <= 1 {0} else {*val - 1},
					"toggle" => *val + 2, 
					_ => 0
				}
			}
		}
	}
	
	let mut part1 = 0;
	let mut part2 = 0;
	
	for i in 0..x_range.len() {
		for j in 0..y_range.len() {
			part1 += first_lights[i][j] * x_diff[i] * y_diff[j];
			part2 += second_lights[i][j] * x_diff[i] * y_diff[j];
		}
	}
	
	let end = start.elapsed().as_micros();
	println!("--------");
	println!("Cutoff: {}",cutoff);
	println!("Part 1: {}",part1);
    println!("Part 2: {}",part2);
    println!("Time: {} μs", end);
}
