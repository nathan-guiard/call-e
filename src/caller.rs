use std::{rc::Rc, cell::RefCell, collections::HashSet};


#[derive(Clone, Debug, Default)]
pub struct Caller {
	pub name: String,
	pub callees: Vec<String>,
}

pub struct Node {
	caller: Caller,
	callees: RefCell<Vec<NodeType>>,
}

enum NodeType {
	Local(Rc<Node>),
	Extern(String),
}


impl Node {
	pub fn create_graph(
		it: impl Iterator<Item = Caller>,
	) -> Vec<Rc<Self>> {
		let a = Rc::new(String::from("test"));

		let mut nodes: Vec<Rc<Self>> =
			it
			.map(|caller| {
				Rc::new(Node {
					caller,
					callees: RefCell::new(Vec::new()),
				})
			})
			.collect();

		for caller in &nodes {
			*caller.callees.borrow_mut() =
				caller.caller.callees
				.iter()
				.map(|name| {
					match nodes.iter().find(|n| n.caller.name == *name) {
						Some(local) => NodeType::Local(local.clone()),
						None => NodeType::Extern(name.clone()),
					}
				})
				.collect()
		}

		nodes
	}

	pub fn visit<F>(&self, mut f: F)
	where
		F: FnMut(Option<&String>,  usize),
	{
		fn visit_inner<F>(this: &Node, visited: &mut HashSet<usize>, rec_no: usize, f: &mut F)
		where
			F: FnMut(Option<&String>, usize)
		{
			let addr = this as *const Node as usize;

			if !visited.insert(addr) {
				f(None, rec_no);
				return;
			}

			f(Some(&this.caller.name), rec_no);

			for callee in this.callees.borrow().iter() {
				match callee {
					NodeType::Extern(name) => f(Some(name), rec_no + 1),
					NodeType::Local(callee) => visit_inner(callee, visited, rec_no + 1, f),
				}
			}

			visited.remove(&addr);
		}

		let mut visited: HashSet<usize> = HashSet::new();
		visit_inner(self, &mut visited, 0, &mut f);
	}

	pub fn display(&self) {
		self.visit(|node, rec_no| {
			for _ in 0..rec_no {
				print!("\t");
			}

			match node {
				Some(n) => println!("{n}"),
				None => println!("..."),
			}
		});
	}

	pub fn name(&self) -> &str {
		&self.caller.name
	}
}