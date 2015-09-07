#[derive(Debug)]
struct Structure(i32);


#[derive(Debug)]
struct Deep(Structure);

fn main() {
	println!("{} days", 30);
	println!("There are {:?} months in a year", 12);
	println!("{1:?}, {0:?} is the {actor:?} name", 
		"Slater",
		"Christian",
		actor="actor's"
		);


	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	println!("{subject} {verb} {predicate}",
		predicate="over the lazy dog.",
		verb= "jumps",
		subject= "The quick brown fox");
	println!("{} of {:b} people know binary, the other half don't", 1, 2);

	println!("My name is {0}, {1} {0}", "Bond", "James");

	println!("Now {:?} will print!", Structure(3));
	println!("Now {:?} will print!", Deep(Structure(7)));
	
}
	

