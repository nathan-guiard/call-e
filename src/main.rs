use std::{env, collections::HashMap};
mod caller;
use caller::Caller;
use std::io::stdin;

fn main() {
    let _argv: Vec<String> = env::args().collect();
	let mut file_read: Vec<String>;
	let _data_parsed: Vec<Caller>;

    // if argv.len() == 1 {
        match read_stdin() {
			Ok(ret) => file_read = ret,
			Err(e) => panic!("Error : {:?}", e)
		}
	// }
    // } else {
    //     read_file();
    // }
	file_read = filtering(file_read);
	let map = parsing(file_read);
	map = assign_callees(map);
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

fn filtering<'a>(mut whole_file: Vec<String>) -> Vec<String> {
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

fn parsing(data: Vec<String>) -> HashMap<String, Caller> {
	let mut functions: HashMap<String, Caller> = HashMap::new();
	let mut current_name: String;
	let mut current_caller: Caller = Caller::new();

	for s in data {
		if s.clone().into_bytes()[0] != b' ' {
			let it = s.split_ascii_whitespace();
			let name = it.last().unwrap();

			if name.len() > 3 {
				if !current_caller.is_empty() {
					functions.insert(current_caller.name.clone(), current_caller.clone());
					current_caller.clear();
				}
				current_name = String::from(&name[1..name.len() - 2]);
				current_caller.name = current_name;
			}
			else {
				panic!("Parsing error: Name of function too short.");
			}
		} else {
			let tab: Vec<&str> = s.split('\t').collect();
			let op;

			if tab.len() < 3 {
				op = "none"
			} else {
				op = tab[2];
			}
			if op.starts_with("call") {
				current_caller.callees_name.push(parse_call_function(op));
			}	
		}
	}
	if !current_caller.is_empty() {
		functions.insert(current_caller.name.clone(), current_caller.clone());
	}

	let it = functions.clone();
	for (key, value) in it {
		let truc = value.callees_name.clone();
		for a in truc {
			println!("{}: {}", key, a);
		}
	}
	return functions;
}

fn parse_call_function(s: &str) -> String {
	let splitted = s.split_ascii_whitespace();
	let name = splitted.last().unwrap();

	return String::from(name[1..name.len() - 1].to_string());
}

fn assign_callees(mut map: HashMap<String, Caller>) -> HashMap<String, Caller> {
	for (name, fnc) in map {
		for callee in fnc.callees_name {
			fnc.callees_struct.push(map.get(callee.clone()));
		}
	}

	return map;
}