#[test] fn links()
{
	let bin = env!("CARGO_BIN_EXE_squarkdown");

	std::process::Command::new(bin)
		.current_dir(std::env::current_dir().unwrap().join("tests/links"))
		.spawn().unwrap();
}
