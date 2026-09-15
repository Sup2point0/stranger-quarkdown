use std::assert_matches;


const BIN: &'static str = env!("CARGO_BIN_EXE_squarkdown");


pub fn squarkup_from(path: &str) -> std::process::ExitStatus
{
	let r = std::process::Command::new(BIN)
		.current_dir(std::env::current_dir().unwrap().join(path))
		.status();

	assert_matches!( r, Ok(..) );
	r.unwrap()
}
