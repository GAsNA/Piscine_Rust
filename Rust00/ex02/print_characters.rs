fn print_characters(s: &str)
{
	for c in s.chars() { println!("{c:?}"); }
}

fn main()
{
	print_characters("Hello!\n");
}
