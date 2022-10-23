// std
use std::{
	fs::OpenOptions,
	io::{BufRead, BufReader, Write},
	process::{Child, Command, Stdio},
	thread::{self, JoinHandle},
	time::Duration,
};

pub struct Process {
	pub child: Option<Child>,
	pub output: Option<JoinHandle<()>>,
}
impl Process {
	pub fn wait(mut self) {
		if let Some(mut child) = self.child.take() {
			child.wait().unwrap();
		}
		if let Some(output) = self.output.take() {
			output.join().unwrap();
		}
	}
}

pub fn run(program: &str, args: &[&str]) {
	Command::new(program)
		.args(args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.output()
		.unwrap();
}

pub fn run_bg(program: &str, args: &[&str], output: Option<&str>) -> Process {
	let mut child = Command::new(program)
		.args(args)
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.unwrap();
	if let Some(output) = output {
		let stderr = child.stderr.take().unwrap();
		let output = output.to_owned();
		let output = thread::spawn(move || {
			let r = BufReader::new(stderr);
			let mut w =
				OpenOptions::new().create(true).write(true).truncate(true).open(output).unwrap();

			r.lines().filter_map(Result::ok).for_each(|l| {
				writeln!(w, "{l}").unwrap();
			});
		});

		return Process { child: Some(child), output: Some(output) };
	}

	Process { child: Some(child), output: None }
}

pub fn sleep(millis: u64) {
	thread::sleep(Duration::from_millis(millis));
}
