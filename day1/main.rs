use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let fin = File::open("input.txt")?;
    let reader = BufReader::new(fin);

	let mut fout = File::create("output.txt")?;

	let mut sum: i32 = 0;

    for line in reader.lines() {
		let line_str: String = line?;
		if 0 < line_str.len() {
			let calorie: i32 = line_str.parse().unwrap();
			sum = sum + calorie;
		} else	{
			let calorie_total: String = sum.to_string();
			write!(fout, "{}\n", calorie_total).ok();
			sum = 0;
		}
	}
    Ok(())
}