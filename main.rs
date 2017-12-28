use std::io::{self, Write};
use std::process::Command;

fn main() {
	shell_loop();
}

fn builtin_exit() -> i8 {
	0
}


fn builtin_help() -> i8 {
	println!("Help:");
	println!("help - Displays this screen");
	println!("exit - Leaves this shell");
	1
}

fn builtin_commands(args: &Vec<&str>) -> i8 {
	match args[0] {
		"help" => builtin_help(),
		"exit" => builtin_exit(),
		_ => -1,
	}
}

fn shell_loop() {
	let mut status = 1;
	while status != 0 {
		let line = read_line();
		let args = split_line(&line);
		status = execute(&args);
	}
}

fn execute(args: &Vec<&str>) -> u8 {

	if args.len() == 0 {
		return 1;
	}

	let result = builtin_commands(&args);
	if result != -1 {
		return result as u8;
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
