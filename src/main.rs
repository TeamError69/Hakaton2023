use regex::Regex;

use parse::library::forklift::Forklift;

use parse::library::work_space::read_config_space;

fn main() {
	let data = vec![
		"Forklift #0 of warehouse #0 have started the task #1 at 2023-10-20 21:18:42.606165",
		"Forklift #0 of warehouse #0 have reach the point K1 at 2024-10-20 21:18:42.606259",
		"Forklift #1 of warehouse #0 have started the task #3 at 2023-10-20 21:18:42.606165",
		"Forklift #1 of warehouse #0 have reach the point K1 at 2024-10-20 21:18:42.606259",
		"Forklift #0 of warehouse #0 have reach the point K2 at 2023-10-20 21:18:48.002174",
		"Forklift #0 of warehouse #0 have reach the point K5 at 2023-10-20 21:18:53.398093",
		"Forklift #0 of warehouse #0 have reach the point K8 at 2023-10-20 21:18:58.794015",
		"Forklift #0 of warehouse #0 have reach the point K9 at 2023-10-20 21:19:09.585807",
		"Forklift #0 of warehouse #0 have reach the target X5 at 2023-10-20 21:19:09.585847",
		"Forklift #0 of warehouse #0 have reach the point K9 at 2023-10-20 21:19:12.001801",
		"Forklift #0 of warehouse #0 have reach the point K8 at 2023-10-20 21:19:17.397756",
		"Forklift #0 of warehouse #0 have reach the point K5 at 2023-10-20 21:19:28.189567",
		"Forklift #0 of warehouse #0 have reach the point K2 at 2023-10-20 21:19:34.238567",
		
		"Forklift #0 of warehouse #0 have started the task #666 at 2023-10-20 21:19:43.200867",
		
		"Forklift #0 of warehouse #0 have reach the point K1 at 2023-10-20 21:19:39.634543",
		"Forklift #0 of warehouse #0 have finished the task #1 at 2023-10-20 21:19:43.200757",
		"Forklift #0 of warehouse #0 have started the task #24 at 2023-10-20 21:19:43.200867",
	];
	let mut forklift = Forklift::try_from(data[0]).unwrap_or_else(|error| {
		println!("Error: {:#?}", error);
		std::process::exit(0x0100)
	});
	println!("{:#?}", forklift);
	for index in 1..(data.len()) {
		if let Ok(val) = Forklift::try_from(data[index]) {
			println!("{:?}", forklift.try_update(val));
			println!("{:#?}", forklift);
		}
	}
}
