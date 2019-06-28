use std::fs::File;
use std::io::BufReader;

use std::collections::HashMap;
use std::error::Error;
use std::process;

use rocksdb::{DB, Options};

fn example() -> Result<(), Box<Error>> {
	// Build the CSV reader and iterate over each record.
	let f = File::open("/home/jp/code/csv/20180521_4M_BANK_PANEL.txt")?;
	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(false)
		.delimiter(b'|')
		.escape(Some(b'\\'))
		.flexible(true)
		.from_reader(BufReader::new(f));
	let mut distinct: HashMap<String, u64> = HashMap::new();
	let mut count: u64 = 0;
	for result in rdr.records() {
		// The iterator yields Result<StringRecord, Error>, so we check the
		// error here.
		let r = result?.get(10).unwrap_or("").to_string();
		let c = match distinct.get(&r) {
			Some(count) => count +1,
			None => 1
		};
		distinct.insert(r, c);
		count += 1;
	}
	let dbPath = "/home/jp/code/csv/rocksdb";
	{
		let db = DB::open_default(dbPath)?;
		for (word, count) in &distinct {
			db.put(word.as_bytes(), count.to_le_bytes())?;
		}
	}
	println!("{} out of {}", distinct.len(), count);
	Ok(())
}

fn main() {
	if let Err(err) = example() {
		println!("error running example: {}", err);
		process::exit(1);
	}
}
