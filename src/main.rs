use std::env;
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