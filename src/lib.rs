use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct HtmlElement {
	pub node_type: String,
	pub text_content: String,
	pub child_nodes: Vec<HtmlElement>,
	pub attributes: HashMap<String, Option<String>>,
}

impl HtmlElement {
	fn new(node_type: String) -> HtmlElement {
		HtmlElement {
			node_type: node_type,
			child_nodes: Vec::new(),
			attributes: HashMap::new(),
			text_content: String::new(),
		}
	}
}

#[derive(PartialEq, Debug)]
enum Tokens {
	Text(String),
	OpenTag(String),
	CloseTag(String),
	SelfCloseTag(String),
	AttributeKey(String),
	AttributeValue(String),
}

impl Tokens {
	fn add(&mut self, c: char) {
		match *self {
			Tokens::Text(ref mut s)
			| Tokens::OpenTag(ref mut s)
			| Tokens::CloseTag(ref mut s)
			| Tokens::SelfCloseTag(ref mut s)
			| Tokens::AttributeKey(ref mut s)
			| Tokens::AttributeValue(ref mut s) => {
				*s = String::from(s.clone()) + &String::from(c);
			}
		}
	}

	fn convert_to_self_close_tag(&mut self) {
		if let Tokens::OpenTag(tag) = &*self {
			*self = Tokens::SelfCloseTag(tag.clone());
		}
	}
}

fn lexer(html: String) -> Vec<Tokens> {
	let mut tokens: Vec<Tokens> = vec![];
	let mut reading = false;
	let mut idx = 0;

	let to_read = html.clone();

	loop {
		let posible_last_token = tokens.last_mut();
		let posible_letter = to_read.chars().nth(idx);
		let posible_next_letter = to_read.chars().nth(idx + 1);

		match (posible_letter, posible_next_letter, posible_last_token) {
			(None, ..) => {
				break tokens;
			}
			(Some('<'), Some(next_letter), _) => {
				let token = if next_letter == '/' {
					Tokens::CloseTag(String::new())
				} else {
					Tokens::OpenTag(String::from(next_letter))
				};

				tokens.push(token);
				reading = true;
				idx += 2;
			}
			(Some('/'), Some('>'), _) => {
				if let Some(token) = tokens.iter_mut().rev().find(|i| match i {
					Tokens::OpenTag(_) => true,
					_ => false,
				}) {
					token.convert_to_self_close_tag();
					reading = false;
				}

				idx += 2;
			}
			(Some('>'), ..) | (Some('"'), ..) => {
				reading = false;
				idx += 1;
			}
			(Some('='), Some('"'), _) => {
				tokens.push(Tokens::AttributeValue(String::new()));
				reading = true;
				idx += 2;
			}
			(Some(' '), Some('/'), _) => {
				reading = false;
				idx +=2;
			}
			(Some(' '), _, Some(last_token @ Tokens::AttributeValue(_))) if reading => {
				last_token.add(' ');
				idx += 1;
			}
			(Some(' '), Some(next_letter), Some(Tokens::OpenTag(_)))
			| (Some(' '), Some(next_letter), Some(Tokens::AttributeValue(_)))
				if next_letter != '/' && next_letter != ' ' =>
			{
				tokens.push(Tokens::AttributeKey(String::new()));
				reading = true;
				idx += 1;
			}
			(Some(letter), _, _) if !reading => {
				tokens.push(Tokens::Text(String::from(letter)));
				reading = true;
				idx += 1;
			}
			(Some(letter), _, Some(last_token)) => {
				last_token.add(letter);
				idx += 1;
			}
			_ => {

			}
		}
	}
}

fn parser(_tokens: Vec<Tokens>) -> Vec<HtmlElement> {
	vec![]
}

/// Returns a parsed html with the value given them
///
/// # Arguments
///
/// * `html` - A string os the html to parse
///
/// # Examples
///
/// ```
/// use crate::rust_simple_parser::HtmlElement;
/// use crate::rust_simple_parser::parse;
/// use std::collections::HashMap;
///
///	let html = String::from("<h1>Olá Marcos</h1>");
///	let parsed = parse(html);
///
///	let expected: Vec<HtmlElement> = vec![
///		HtmlElement {
///			text_content: String::from("Olá Marcos"),
///			node_type: String::from("h1"),
///			attributes: HashMap::new(),
///			child_nodes: Vec::new(),
///		}
///	];
///
/// assert_eq!(expected, parsed);
/// ```
pub fn parse(html: String) -> Vec<HtmlElement> {
	let tokens = lexer(html);
	println!("{:#?}", tokens);

	let parsed = parser(tokens);

	parsed
}

#[cfg(test)]
mod tests {
	use crate::parse;
	use crate::HtmlElement;
	use std::collections::HashMap;

	#[test]
	fn parse_text() {
		let text_content = String::from("Olá Marcos");
		let lexer = parse(text_content.clone());

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("text"),
			text_content: text_content,
			attributes: HashMap::new(),
			child_nodes: Vec::new(),
		}];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node() {
		let text_content = String::from("Olá Marcos");
		let html_h1 = String::from("<h1>") + &text_content + &String::from("</h1>");
		let lexer = parse(html_h1);

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("h1"),
			text_content: text_content,
			attributes: HashMap::new(),
			child_nodes: Vec::new(),
		}];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node_with_child() {
		let text_content = String::from("Olá Marcos");
		let html_h1_b = String::from("<h1><b>") + &text_content + &String::from("</b></h1>");
		let lexer = parse(html_h1_b);

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("h1"),
			text_content: String::new(),
			attributes: HashMap::new(),
			child_nodes: vec![HtmlElement {
				node_type: String::from("b"),
				text_content: text_content,
				attributes: HashMap::new(),
				child_nodes: Vec::new(),
			}],
		}];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node_with_child_and_text() {
		let text_content_h1 = String::from("Olá ");
		let text_content_b = String::from("Marcos");
		let html_b = String::from("<b>") + &text_content_b + &String::from("</b>");
		let html_h1_b = String::from("<h1>") + &text_content_h1 + &html_b + &String::from("</h1>");
		let lexer = parse(html_h1_b);

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("h1"),
			text_content: text_content_h1,
			attributes: HashMap::new(),
			child_nodes: vec![HtmlElement {
				node_type: String::from("b"),
				text_content: text_content_b,
				attributes: HashMap::new(),
				child_nodes: Vec::new(),
			}],
		}];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_self_close_nodes() {
		let html_form = String::from("<form><input /></form>");
		let lexer = parse(html_form);

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("form"),
			text_content: String::new(),
			attributes: HashMap::new(),
			child_nodes: vec![HtmlElement {
				node_type: String::from("input"),
				text_content: String::new(),
				attributes: HashMap::new(),
				child_nodes: Vec::new(),
			}],
		}];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_nodes_with_attributes() {
		let html_form = String::from("<input id=\"teste\" />");
		let lexer = parse(html_form);

		let mut attributes: HashMap<String, Option<String>> = HashMap::new();

		attributes.insert("id".to_string(), Some(String::from("teste")));

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("input"),
			text_content: String::new(),
			attributes: attributes,
			child_nodes: vec![],
		}];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_nodes_with_attributes_spaced() {
		let html_form = String::from("<input id=\"teste\" />");
		let lexer = parse(html_form);

		let mut attributes: HashMap<String, Option<String>> = HashMap::new();

		attributes.insert("id".to_string(), Some(String::from("teste")));

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("input"),
			text_content: String::new(),
			attributes: attributes,
			child_nodes: vec![],
		}];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node_with_sinbling_and_child() {
		let text_content_h1 = String::from("Olá ");
		let text_content_b = String::from("Marcos");
		let text_content_h2 = String::from("Sou Frontend");
		let html_b = String::from("<b>") + &text_content_b + &String::from("</b>");
		let html_h1_b = String::from("<h1>") + &text_content_h1 + &html_b + &String::from("</h1>");
		let html_h1_b_h2 =
			html_h1_b + &String::from("<h2>") + &text_content_h2 + &String::from("</h2>");
		let html_header = String::from("<header>") + &html_h1_b_h2 + &String::from("</header>");
		let lexer = parse(html_header);

		let expected: Vec<HtmlElement> = vec![HtmlElement {
			node_type: String::from("header"),
			text_content: String::new(),
			attributes: HashMap::new(),
			child_nodes: vec![
				HtmlElement {
					node_type: String::from("h1"),
					text_content: text_content_h1,
					attributes: HashMap::new(),
					child_nodes: vec![HtmlElement {
						node_type: String::from("b"),
						text_content: text_content_b,
						attributes: HashMap::new(),
						child_nodes: Vec::new(),
					}],
				},
				HtmlElement {
					node_type: String::from("h2"),
					text_content: text_content_h2,
					attributes: HashMap::new(),
					child_nodes: Vec::new(),
				},
			],
		}];

		assert_eq!(expected, lexer);
	}
}
