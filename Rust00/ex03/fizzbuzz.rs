fn fizzbuzz(i : u32)
{
	if i == 101 { return ; }
	print!("	puts(\"");
	if i % 3 == 0 { print!("fizz"); }
	if i % 5 == 0 { print!("buzz"); }
	if i % 3 != 0 && i % 5 != 0 { print!("{}", i); }
	println!("\");");
	fizzbuzz(i + 1);
}

fn main()
{
	println!("#include <stdio.h>\n\nint	main(void)\n{{");
	fizzbuzz(1);
	println!("}}");
}
