use std::{env, collections::HashMap};
mod caller;
use caller::Caller;
use caller::Node;
use std::io::stdin;
use std::io::Read;

fn is_a_tty(fd: i32) -> bool {
	extern "C" {
		fn isatty(fd: std::ffi::c_int) -> std::ffi::c_int;
	}

	// SAFETY:
	//  This function is always safe to call.
	unsafe {isatty(fd) != 0 }
}

fn main() {
    let _argv: Vec<String> = env::args().collect();
	let _data_parsed: Vec<Caller>;

    // if argv.len() == 1 {
		let mut file_read = String::new();
        match stdin().read_to_string(&mut file_read) {
			Ok(_) => (),
			Err(e) => panic!("Error : {:?}", e)
		}

			println!("stdin: {}", is_a_tty(0));
			println!("stdout: {}", is_a_tty(1));

	// }
    // } else {
    //     read_file();
    // }
	// file_read = filtering(file_read);
	let callers = CallerList::new(file_read.lines());
	let nodes = Node::create_graph(callers);

	println!("FUNCTIONS:");
	for node in &nodes {
		println!("{}", node.name());
	}
	
	println!();
	println!();

	nodes.iter().find(|node| node.name() == "main").unwrap().display();

	// map = assign_callees(map);
}

fn read_stdin() -> std::io::Result<Vec<String>> {
    let mut open_result;
    let mut buff = String::new();
    let mut whole_file: Vec<String> = Vec::new();

    loop {
		open_result = stdin().read_line(&mut buff);
		if buff.len() > 0 {
			buff.truncate(buff.len() - 1);
		}
		match open_result {
			Ok(size) => match size {
				0 => { whole_file.push(buff.clone()); break; },
				_ => whole_file.push(buff.clone()),
			},
			Err(e) => return Err(e),
		}
		buff.clear();
    }
    Ok(whole_file)
}

fn _read_file() {
    println!("Called read_file");
}

fn filtering(mut whole_file: Vec<String>) -> Vec<String> {
	let mut section_text: bool = false;
	let mut _current_function: Caller;
	let mut filtered: Vec<String> = vec![];
	let panic_str: &str = "Parsing error: Multiple .text disassembly sections";

	for s in whole_file.clone() {
		match (section_text,
			s == "Disassembly of section .text:",
			s.starts_with("Disassembly of section")) {
				(false, false, _)	=>	continue,
				(false, true, _)	=>	section_text = true,
				(true, false, true) =>	break,
				(true, false, false) =>	{
					if !s.is_empty() {
						println!("{s}");
						filtered.push(s);
					}
				}
				(true, true, _)		=>	panic!("{panic_str}"),
		}
	}
	whole_file = filtered;
	return whole_file;
}

struct CallerList<I> {
	lines: I,
}

impl<'a, I> CallerList<I>
where
	I: Iterator<Item = &'a str>,
{
	pub fn new(mut lines: I) -> Self {
		for line in &mut lines {
			if line.starts_with("Disassembly of section .text:") {
				break;
			}
		}

		Self { lines }
	}
}

impl<'a, I> Iterator for CallerList<I>
where
	I: Iterator<Item = &'a str>,
{
	type Item = Caller;

	fn next(&mut self) -> Option<Self::Item> {
		// Find the name.
		let almost_name = self
			.lines
			.find(|line| !line.is_empty())?
			.split_ascii_whitespace()
			.last()
			.unwrap();

		let name = almost_name[1..almost_name.len() - 2].to_string();

		// Find the callees.
		let callees: Vec<String> =
			(&mut self.lines)
			.take_while(|line| !line.is_empty())
			.filter_map(|line| {
				line.split('\t')
					.nth(2)
					.and_then(parse_call_function)
			})
			.map(str::to_string)
			.collect();

		Some(Caller {
			name,
			callees
		})
	}
}

fn parse_call_function(s: &str) -> Option<&str> {
	if !s.starts_with("call") {
		return None;
	}
	
	let splitted = s.split_ascii_whitespace();
	let name = splitted.last().unwrap();

	if name == "call" {
		panic!("parsing error no fnc name");
	}

	return Some(&name[1..name.len() - 1]);
}

// fn assign_callees(mut map: HashMap<String, Caller>) -> HashMap<String, Caller> {
// 	for (name, fnc) in map {
// 		for callee in fnc.callees_name {
// 			fnc.callees_struct.push(map.get(callee.clone()));
// 		}
// 	}

// 	return map;
// }