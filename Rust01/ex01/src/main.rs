fn	min<'a>(a: &'a i32, b: &'a i32) -> &'a i32
{
	if *a < *b { return a; }
	return b;
}

fn main() {
	let	a: i32 = -9;
	let	b: i32 = 7;
	let c: i32 = 42;
    println!("{}", min(&a, &b));
    println!("{}", min(&a, &c));
    println!("{}", min(&b, &c));
}
