
use rust_simple_parser::parse;

fn main() {
	println!("{:#?}", parse(String::from("Olá Marcos")));
	println!("{:#?}", parse(String::from("<h1>Olá Marcos</h1>")));
	println!("{:#?}", parse(String::from("<h1><b>Olá Marcos</b></h1>")));
	println!("{:#?}", parse(String::from("<h1>Olá <b>Marcos</b></h1>")));
	println!("{:#?}", parse(String::from("<header><h1>Olá <b>Marcos</b></h1><h2>Sou Frontend</h2></header>")));
}
