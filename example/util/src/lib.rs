// std
use std::{
	env,
	fs::OpenOptions,
	io::{BufRead, BufReader, Write},
	path::Path,
	process::{Child, Command, Stdio},
	thread::{self, JoinHandle},
	time::Duration,
};

pub struct ExampleNodeEnv {
	pub base_dir: String,
	pub repository_dir: String,
	pub executable_path: String,
	pub log_path: String,
	pub data_dir: String,
}
impl ExampleNodeEnv {
	pub fn setup(with_compiling: bool) -> Self {
		let current_dir = env::current_dir().unwrap();
		let base_dir = "/tmp/subalfred-example".into();
		let repository_dir = format!("{base_dir}/substrate-node-template");
		let executable_path = format!("{repository_dir}/target/debug/node-template");
		let log_path = format!("{base_dir}/log");
		let data_dir = format!("{base_dir}/data");

		if !Path::new(&repository_dir).exists() {
			// Clone the repository.
			run(
				"git",
				&[
					"clone",
					"https://github.com/substrate-developer-hub/substrate-node-template.git",
					&repository_dir,
				],
			);
		}

		env::set_current_dir(&repository_dir).unwrap();
		// Build the node template.
		//
		// Make sure you have met the compiling requirement.
		// Such as, `gcc`, `llvm`, `wasm32-unknown-unknown`, `protobuf` and etc.
		if with_compiling {
			run("cargo", &["build"]);
		}
		env::set_current_dir(current_dir).unwrap();

		Self { base_dir, repository_dir, executable_path, log_path, data_dir }
	}
}

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

	pub fn kill(mut self) {
		if let Some(mut child) = self.child.take() {
			child.kill().unwrap();
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
