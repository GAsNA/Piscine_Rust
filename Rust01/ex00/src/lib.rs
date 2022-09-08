fn	add(a: &i32, b: i32) -> i32 {
	a + b
	//*a + b
}

fn	add_assign(a: &mut i32, b: i32) {
	*a = *a + b;
}

fn main() {
	let	a: i32 = 9;
	let b: i32 = 7;
    println!("{}", add(&a, b));
	let mut c: i32 = 7;
	let d: i32 = 42;
	add_assign(&mut c, d);
	println!("{}", c);
}
