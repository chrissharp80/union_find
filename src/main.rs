use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod uf;

fn main() {
	let filename = "/Users/lcsharp/Desktop/testfile.txt";
	println!("In file {}", filename);
	let mut first = true;
	let f = File::open(&filename).expect("file not found");
	let mut a: i32;
	let mut b: i32;
	let file = BufReader::new(&f);
	for line in file.lines() {
		let l = line.unwrap();
		if first {
			let p: i32 = l.parse().unwrap();
			println!("{}", p);
			first = false;
			continue;		
		}
		let mut parts = l.split_whitespace().map(|l| l.parse::<i32>());
		a = parts.next().unwrap().unwrap();
		b = parts.next().unwrap().unwrap();
		println!("{} {}", a, b);
	}
	
}
