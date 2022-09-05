fn collatz(start: u32)
{
	let mut n: u32 = start;

	println!("{n}");
	while n > 1
	{
		if n % 2 == 0 { n /= 2; } else { n = 3 * n + 1; }
		println!("{n}");
	}
}

fn main()
{
	collatz(3);
}
