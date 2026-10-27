use std::process::{ExitCode, exit};


pub fn error_if<T, I>(result : Result<T, I>) -> T {
	match result {
		Ok(res) => res,
		Err(err) => {
			println!("ERROR!\n");
			exit(1)
		}
	}
}
