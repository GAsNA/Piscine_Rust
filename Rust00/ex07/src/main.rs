fn main() {
	let tab: [&str; 5] = ["cool ring", "super computer", "stack of old books", "lil house", "incredible motorcycle"];

	let num_i32: i32 = ftkit::random_number(0..5);
	let num: usize = num_i32 as usize;
	let price: i32 = ftkit::random_number(0..=100);
	let mut devine: i32 = -1;

	println!("Here is a fabulous '{}'.", tab[num]);
	while devine != price
	{
		devine = ftkit::read_number();
		if price > devine { println!("A '{}' costs more than that!", tab[num]); }
		else if price < devine { println!("A '{}' isn't worth that much money!", tab[num]); }
		else { println!("Congrats! That '{}' is worth ${}", tab[num], price); }
	}
}
