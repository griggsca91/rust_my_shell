use std::io;

fn main() {
	shell_loop();
}

fn shell_loop() {
	loop {

		let line = read_line();
		let args = split_line(&line);
		let result = execute(&args);
		println!("Howdy howdy: {:?}", args);

	}
}

fn execute(args: &Vec<&str>) -> u8 {

	return 1;
}

fn split_line(line: &String) -> Vec<&str> {
	line.split_whitespace().collect()
}

fn read_line() ->  String {
		let mut input = String::new();
		
		io::stdin().read_line(&mut input)
			.expect("Failed to read line");
		
		return input;
}
