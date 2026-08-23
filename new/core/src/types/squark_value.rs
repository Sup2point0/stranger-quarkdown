pub enum SquarkValue {
	Bool(bool),
	Number(i32),
	String(String),
	List(Vec<SquarkValue>),
}
