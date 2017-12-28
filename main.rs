use std::io::{self, Write};
use std::process::Command;

fn main() {
	shell_loop();
}

fn shell_loop() {
	loop {
		let line = read_line();
		let args = split_line(&line);
		let result = execute(&args);
	}
}

fn execute(args: &Vec<&str>) -> u8 {

	if (args.len() == 0) {
		return 1;
	}

	let output = Command::new(args[0])
						.args(args[1..].iter())
						.output()
						.expect("failed to execute process");
	print!("{}", String::from_utf8_lossy(&output.stdout));
	return 1;
}

fn split_line(line: &String) -> Vec<&str> {
	return line.split_whitespace().collect();
}

fn read_line() ->  String {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut input = String::new();
		
		io::stdin().read_line(&mut input)
			.expect("Failed to read line");
		
		return input;
}
