use std::env;

fn main() {
	if let Some(ip) = env::args().nth(1) {
		let split = ip
			.split(".")
			.map(|x| x.parse::<u32>().expect("Invalid IPv4 address"))
			.fold(0, |x, y| (x << 0x8) | y);

		println!("{:#010x}", split);
	} else {
		println!("Please specify an IPv4 address to convert");
	}
}