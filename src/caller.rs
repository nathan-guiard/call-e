pub struct Caller {
	name: String,
	callee_struct: Vec<Caller>,
	callee_name: Vec<String>,
}

impl Caller {
	fn clear() -> Self {
		Caller {
			name: String::new(),
			callee_struct: vec![],
			callee_name: vec![],
		}
	}
}