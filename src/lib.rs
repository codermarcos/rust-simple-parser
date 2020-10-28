use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Actions {
	EmptyState,
	ReadingInnerTag,
	ReadingAttributes,
	ReadingOpenTagName,
	ReadingCloseTagName,
	ReadingAttributesValue(String),
}

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

const SELF_CLOSE_TAGS: [&str; 14] = [
	"area",
	"base",
	"br", 
	"col",
	"embed",
	"hr",
	"img",
	"input",
	"link",
	"meta",
	"param",
	"source",
	"track",
	"wbr",
];

fn parse_shallow(html: String) -> (Vec<HtmlElement>, usize) {
	let mut parsed: Vec<HtmlElement> = vec![];
	let mut state: Vec<Actions> = vec![];
	let mut reading = String::new();
	let mut idx = 0;

	let to_read = html.clone();

	loop {
		if let Some(last_state) = state.last().clone() {
			if let Some(letter) = to_read.chars().nth(idx) {
				match letter {
					'<' => {
						if let Some(next_letter) = to_read.chars().nth(idx + 1) {
							if next_letter == '/' {
								state.push(Actions::ReadingCloseTagName);

								if reading.capacity() > 0 {
									if parsed.len() > 0 {
										parsed[0].text_content = reading.clone(); 
									} else {
										let mut element = HtmlElement::new(String::from("text"));
										element.text_content = reading.clone();
										parsed.push(element);
									}
									
									reading = String::new();
								}
								
								idx += 2;
							} else {
								if last_state == &Actions::ReadingInnerTag {
									let last_idx_parsed = parsed.len() - 1;
									
									if reading.capacity() > 0 {
										parsed[last_idx_parsed].text_content = reading;
										reading = String::new();
										idx += 1;
									}

									let deep_read = (&to_read[idx..]).to_string(); 
									let (elements, readed) = parse_shallow(deep_read);

									for element in elements {
										parsed[last_idx_parsed].child_nodes.push(element);
									}

									idx += readed;
								} else {
									state.push(Actions::ReadingOpenTagName);
									idx += 1;
								}
							}
						}
					}
					'>' => {
						match last_state {
							Actions::ReadingOpenTagName => {
								let element = HtmlElement::new(reading.clone());
								let tag_name: &str = &reading;
								parsed.push(element);
								state.pop();

								if !SELF_CLOSE_TAGS.contains(&tag_name) {
									state.push(Actions::ReadingInnerTag);
								}

								reading = String::new();
							}
							Actions::ReadingAttributes => {
								reading = String::new();
								
								state.pop();
								idx += 1;	
							}
							Actions::ReadingCloseTagName => {
								state.pop();
								break (parsed, idx);
							}
							_ => {}
						}

						idx += 1;
					}
					'/' => {
						if let Some(next_letter) = to_read.chars().nth(idx + 1) {
							if next_letter == '>' {
								match last_state {
									Actions::ReadingOpenTagName => {
										let element = HtmlElement::new(reading.clone());
										reading = String::new();
										parsed.push(element);
										
										state.pop();
										idx += 2;	
									}
									Actions::ReadingAttributes => {
										reading = String::new();
										
										state.pop();
										idx += 1;	
									}
									_ => {}
								}								
							}
						}
					}
					'=' | '"' => {	
						match last_state {
							Actions::ReadingAttributes => {							
								if reading.capacity() > 0 {
									state.push(Actions::ReadingAttributesValue(reading));
									reading = String::new();
								}
							}
							Actions::ReadingAttributesValue(key) => {
								let last_idx_parsed = parsed.len() - 1;
									
								if reading.capacity() > 0 {
									parsed[last_idx_parsed].attributes.insert(key.to_string(), Some(reading.clone()));
									reading = String::new();
									state.pop();
								}
							}
							_ => {
								reading = reading + letter.to_string().as_str();
							}
						}
						idx += 1;
					}
					_ => {
						match last_state {
							Actions::ReadingOpenTagName => {
								if letter == ' ' {
									state.pop();
									parsed.push(HtmlElement::new(reading.clone()));
									state.push(Actions::ReadingAttributes);
									reading = String::new();
								} else {
									reading = reading + letter.to_string().as_str();
								}
							}
							_ => {
								reading = reading + letter.to_string().as_str();
							}
						}

						idx += 1;
					}
				}
			} else {
				if reading.capacity() > 0 {
					let mut element = HtmlElement::new(String::from("text"));
					element.text_content = reading.clone();
					parsed.push(element);
				}

				break (parsed, idx);
			}
		} else {
			state.push(Actions::EmptyState);
		}
	}
}

pub fn parse(html: String) -> Vec<HtmlElement> {
	let (parsed, _) = parse_shallow(html);
	
	parsed
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use crate::HtmlElement;
	use crate::parse;

	#[test]
	fn parse_text() {
		let text_content = String::from("Olá Marcos");
		let lexer = parse(text_content.clone());

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("text"),
				text_content: text_content,
				attributes: HashMap::new(),
				child_nodes: Vec::new(),
			}
		];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node() {
		let text_content = String::from("Olá Marcos");
		let html_h1 = String::from("<h1>") + &text_content + &String::from("</h1>");
		let lexer = parse(html_h1);

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("h1"),
				text_content: text_content,
				attributes: HashMap::new(),
				child_nodes: Vec::new(),
			}
		];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node_with_child() {
		let text_content = String::from("Olá Marcos");
		let html_h1_b = String::from("<h1><b>") + &text_content + &String::from("</b></h1>");
		let lexer = parse(html_h1_b);

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("h1"),
				text_content: String::new(),
				attributes: HashMap::new(),
				child_nodes: vec![
					HtmlElement {
						node_type: String::from("b"),
						text_content: text_content,
						attributes: HashMap::new(),
						child_nodes: Vec::new(),
					}		
				],
			}
		];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node_with_child_and_text() {
		let text_content_h1 = String::from("Olá ");
		let text_content_b = String::from("Marcos");
		let html_b = String::from("<b>") + &text_content_b + &String::from("</b>");
		let html_h1_b = String::from("<h1>") + &text_content_h1 + &html_b + &String::from("</h1>");
		let lexer = parse(html_h1_b);

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("h1"),
				text_content: text_content_h1,
				attributes: HashMap::new(),
				child_nodes: vec![
					HtmlElement {
						node_type: String::from("b"),
						text_content: text_content_b,
						attributes: HashMap::new(),
						child_nodes: Vec::new(),
					}		
				],
			}
		];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_self_close_nodes() {
		let html_form = String::from("<form><input /></form>");
		let lexer = parse(html_form);

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("form"),
				text_content: String::new(),
				attributes: HashMap::new(),
				child_nodes: vec![
					HtmlElement {
						node_type: String::from("input"),
						text_content: String::new(),
						attributes: HashMap::new(),
						child_nodes: Vec::new(),
					},
				],
			}
		];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_nodes_with_attributes() {
		let html_form = String::from("<input id=\"teste\" />");
		let lexer = parse(html_form);

		let mut attributes: HashMap<String, Option<String>> = HashMap::new();

		attributes.insert("id".to_string(), Some(String::from("teste")));

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("input"),
				text_content: String::new(),
				attributes: attributes,
				child_nodes: vec![],
			}
		];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_nodes_with_attributes_spaced() {
		let html_form = String::from("<input id=\"teste\" />");
		let lexer = parse(html_form);

		let mut attributes: HashMap<String, Option<String>> = HashMap::new();

		attributes.insert("id".to_string(), Some(String::from("teste")));

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("input"),
				text_content: String::new(),
				attributes: attributes,
				child_nodes: vec![],
			}
		];

		assert_eq!(expected, lexer);
	}

	#[test]
	fn parse_node_with_sinbling_and_child() {
		let text_content_h1 = String::from("Olá ");
		let text_content_b = String::from("Marcos");
		let text_content_h2 = String::from("Sou Frontend");
		let html_b = String::from("<b>") + &text_content_b + &String::from("</b>");
		let html_h1_b = String::from("<h1>") + &text_content_h1 + &html_b + &String::from("</h1>");
		let html_h1_b_h2 = html_h1_b + &String::from("<h2>") + &text_content_h2 + &String::from("</h2>");
		let html_header = String::from("<header>") + &html_h1_b_h2 + &String::from("</header>");
		let lexer = parse(html_header);

		let expected: Vec<HtmlElement> = vec![
			HtmlElement {
				node_type: String::from("header"),
				text_content: String::new(),
				attributes: HashMap::new(),
				child_nodes: vec![
					HtmlElement {
						node_type: String::from("h1"),
						text_content: text_content_h1,
						attributes: HashMap::new(),
						child_nodes: vec![
							HtmlElement {
								node_type: String::from("b"),
								text_content: text_content_b,
								attributes: HashMap::new(),
								child_nodes: Vec::new(),
							},		
						],
					},
					HtmlElement {
						node_type: String::from("h2"),
						text_content: text_content_h2,
						attributes: HashMap::new(),
						child_nodes: Vec::new(),
					},
				],
			}
		];

		assert_eq!(expected, lexer);
	}
}