
use rust_simple_parser::parse;

fn main() {
	println!("simple text ---------------------------------------");

	println!("{:#?}", parse(String::from("Olá Marcos")));

	println!("tag close -----------------------------------------");

	println!("{:#?}", parse(String::from("<h1>Olá Marcos</h1>")));

	println!("child tags ----------------------------------------");

	println!("{:#?}", parse(String::from("<h1><b>Olá Marcos</b></h1>")));

	println!("child tags with text ------------------------------");

	println!("{:#?}", parse(String::from("<h1>Olá <b>Marcos</b></h1>")));

	println!("self close tags -----------------------------------");

	println!("{:#?}", parse(String::from("<form><input/></form>")));

	println!("siblings tags -------------------------------------");

	println!("{:#?}", parse(String::from("<header><h1>Olá <b>Marcos</b></h1><h2>Sou Frontend</h2></header>")));

	println!("tag with attributes -------------------------------");

	println!("{:#?}", parse(String::from("<input id=\"teste\" />")));

	println!("-----------------------------------------");
}
