use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

struct Config {
	query: String,
	filename: String,
	case_sensitive: bool,
}

impl Config {
	// fn new(args: &[String]) -> Result<Config, &'static str> {
	fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		// if args.len() < 3 {
		// 	return Err("not enough arguments")
		// }
		// let query = args[1].clone();
		// let filename = args[2].clone();
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string"),
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file name"),
		};

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		Ok(Config { query, filename, case_sensitive })
	}
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	// let mut results = Vec::new();
	// for line in contents.lines() {
	// 	if line.contains(query) {
	// 		results.push(line);
	// 	}
	// }
	// results
	contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	// let mut results = Vec::new();
	// for line in contents.lines() {
	// 	if line.to_lowercase().contains(&query) {
	// 		results.push(line);
	// 	}
	// }
	// results
	contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

fn run(config: Config) -> Result<(), Box<Error>> {
	// let mut f = File::open(config.filename).expect("file not found");
	let mut f = File::open(config.filename)?;

	let mut contents = String::new();
	// f.read_to_string(&mut contents).expect("something went wrong reading");
	f.read_to_string(&mut contents)?;

	// for line in search(&config.query, &contents) {
	// 	println!("{}", line);
	// }

	let result = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in result {
		println!("{}", line.trim());
	}

	Ok(())
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	let config = Config::new(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}