fn min(a: i32, b: i32) -> i32
{
	if a < b { a } else { b }
}

fn main()
{
	println!("{}", min(-1, 0));
	println!("{}", min(0, -1));
	println!("{}", min(0, 0));
	println!("{}", min(42, 21));
	println!("{}", min(-42, -21));
}
