use std::string;

#[derive(Clone)]
pub struct Caller {
	pub name: String,
	pub callees_struct: Vec<Callee>,
	pub callees_name: Vec<String>,
}

#[derive(Clone)]
pub struct Callee {
	function: Caller,
	address_of_call: String,
}

impl Caller {
	pub fn clear(&mut self) {
		self.name = String::new();
		self.callees_name = vec![];
		self.callees_struct = vec![];
	}

	pub fn new() -> Self {
		Caller {
			name: String::new(),
			callees_struct: vec![],
			callees_name: vec![],
		}
	}

	pub fn is_empty(&self) -> bool {
		self.name.is_empty()
	}
}