use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Actions {
	EmptyState,
	ReadingInnerTag,
	ReadingSomething,
	ReadingAttributes,
	ReadingOpenTagName,
	ReadingCloseTagName,
}

#[derive(Debug)]
pub struct HtmlElement {
	pub node_type: String,
	pub text_content: String,
	pub child_nodes: Vec<HtmlElement>,
	pub attributes: HashMap<String, String>,
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

/*
pub fn parse_try_0(html: String) -> Vec<HtmlElement> {
	let mut parsed: Vec<HtmlElement> = vec![];

	let mut tag_opened = 0;
	let mut reading = String::new();
	let mut actions: Vec<Actions> = vec![];
	let mut readed_but_not_tokened = String::new();

	for letter in html.chars() {
		let length_tokened = parsed.len();
		let length_actions = actions.len();
		let last_action = actions.last().clone();
		let last_idx_tokened = if length_tokened > 0 { length_tokened - 1 } else { 0 };

		match letter.to_string().as_str() {
			"<" => {
				actions.push(Actions::ReadingOpenOrClose);
				reading = String::new();

				if length_tokened > 0 && readed_but_not_tokened.capacity() != 0 {
					parsed[last_idx_tokened].text_content = readed_but_not_tokened.clone();
					readed_but_not_tokened = String::new();
				}
			}
			">" => {
				let to_read = reading.clone();

				if let Some(last) = last_action {
					match last {
						Actions::ReadingOpenOrClose => {
							let element = HtmlElement {
								tag_name: String::from(to_read),
								text_content: String::new(),
								attributes: HashMap::new(),
								child_nodes: vec![],
							};

							if tag_opened == 0 {
								parsed.push(element);
								tag_opened += 1;
							} else {
								parsed[last_idx_tokened].child_nodes.push(element);
							}
						}
						Actions::ReadingClosedTag => {
							tag_opened -= 1;
						}
						_ => {

						}
					}

					actions.truncate(length_actions - 1);
				}
				reading = String::new();
			}
			"/" => {
				if length_actions > 0 {
					actions.truncate(length_actions - 1);
					actions.push(Actions::ReadingClosedTag);

					reading = String::new();
				}
			}
			_ => {

				match last_action {
					Some(last) =>  {
						reading = format!("{}{}", reading, letter.to_string());
						let to_read = reading.clone();

						match last {
							Actions::ReadingOpenOrClose => {
								if letter.to_string().as_str() == " " {
									let element = HtmlElement {
										tag_name: String::from(to_read),
										text_content: String::new(),
										attributes: HashMap::new(),
										child_nodes: vec![],
									};
									if tag_opened == 0 {
										parsed.push(element);
										tag_opened += 1;
									} else {
										parsed[last_idx_tokened].child_nodes.push(element);
									}
									reading = String::new();
								}
							}
							Actions::ReadingClosedTag => {
								if readed_but_not_tokened.capacity() != 0 {
									parsed[last_idx_tokened].text_content = readed_but_not_tokened.clone();
									readed_but_not_tokened = String::new();
								}
							}
							Actions::ReadingSomething => {
								readed_but_not_tokened = to_read;
							}
						}
					}
					None => {
						actions.push(Actions::ReadingSomething);
					}
				}
			}
		}
	}

	parsed
}
*/

/*
pub fn parse_try_1(html: String) -> Option<HtmlElement> {
	let mut parsed: Option<HtmlElement> = None;
	let mut to_read = html.clone();

	let mut readed = String::new();
	let mut reading = String::new();
	let mut actions: Vec<Actions> = vec![];

	fn createElement(tag_name: String) -> HtmlElement {
		HtmlElement {
			tag_name: tag_name,
			child_nodes: vec![],
			attributes: HashMap::new(),
			text_content: String::new(),
		}
	}
	for letter in html.chars() {
		let last_action = actions.last().clone();
		readed = readed + letter.to_string().as_str();

		match letter {
			'<' => {
				actions.push(Actions::ReadingTagName);
				reading = String::new();
			}
			'>' => {
				if let Some(last) = last_action {
					match last {
						Actions::ReadingTagName => {
							parsed = Some(createElement(reading));
							actions.pop();
							break;
						}
						_ => {}
					}
				}
			}
			_ => {
				if let Some(last) = last_action {
					match last {
						Actions::ReadingTagName => {
							if letter == ' ' {
								parsed = Some(createElement(reading));
								actions.pop();
								break;
							} else {
								reading = reading + letter.to_string().as_str();
							}
						}
						Actions::ReadingSomething => {
							reading = reading + letter.to_string().as_str();
						}
					}
				} else {
					actions.push(Actions::ReadingSomething);
					reading = reading + letter.to_string().as_str();
				}
			}
		}

		to_read = to_read[readed.capacity()..].to_string();
	}

	for letter in to_read.chars().rev() {
		let last_action = actions.last().clone();
		readed = readed + letter.to_string().as_str();

		match letter {
			'>' => {
				actions.push(Actions::ReadingTagName);
				reading = String::new();
			}
			'<' => {
				if let Some(last) = last_action {
					match last {
						Actions::ReadingTagName => {
							parsed = Some(createElement(reading));
							actions.pop();
							break;
						}
						_ => {}
					}
				}
			}
			_ => {
				if let Some(last) = last_action {
					match last {
						Actions::ReadingTagName => {
							if letter == '/' {
								parsed = Some(createElement(reading));
								actions.pop();
								break;
							} else {
								reading = reading + letter.to_string().as_str();
							}
						}
						Actions::ReadingSomething => {
							reading = reading + letter.to_string().as_str();
						}
					}
				} else {
					actions.push(Actions::ReadingSomething);
					reading = reading + letter.to_string().as_str();
				}
			}
		}

		to_read = to_read[readed.capacity()..].to_string();
	}

	parsed
}
*/

/*
pub fn parse_try_2(html: String) -> Option<HtmlElement> {
	let mut parsed: Option<HtmlElement> = None;
	let mut to_read = html.clone();

	fn create_element(tag_name: String) -> Option<HtmlElement> {
		Some(HtmlElement {
			tag_name: tag_name,
			child_nodes: vec![],
			attributes: HashMap::new(),
			text_content: String::new(),
		})
	}

	let mut reading = String::new();
	let mut actions: Vec<Actions> = vec![];

	for letter in html.chars() {
		let last_action = actions.last().clone();

		match letter {
			'<' => {
				if let Some(last) = last_action {
					match last {
						Actions::FindingTagClose => {
							actions.pop();
							reading = String::new();
							actions.push(Actions::ReadingChildTagName);
						}
						Actions::ReadingSomething => {
							if let Some(ref mut parsed) = parsed {
								parsed.text_content = reading.to_string().clone();
								reading = String::new();
								actions.pop();
								actions.push(Actions::ReadingChildTagName);
							}
						}
						_ => {}
					}
				} else {
					actions.push(Actions::ReadingOpenTagName);
					to_read = to_read.chars().skip(1).collect();
					reading = String::new();
				}
			}
			'>' => {
				to_read = to_read.chars().skip(1).collect();
				if let Some(last) = last_action {
					match last {
						Actions::ReadingOpenTagName => {
							parsed = create_element(reading.clone());
							reading = String::new();
							actions.pop();
							actions.push(Actions::FindingTagClose);
						}
						Actions::ReadingCloseTagName => {
							actions.pop();
							if actions.len() == 1 && actions[0] == Actions::FindingTagClose {
								actions.pop();
								break;
							}
						}
						Actions::ReadingAttributes => {
							actions.pop();
							actions.push(Actions::FindingTagClose);
						}
						_ => {}
					}
				}
			}
			'/' => {
				if let Some(last) = last_action {
					match last {
						Actions::ReadingChildTagName => {
							actions.pop();
							actions.push(Actions::ReadingCloseTagName);
						}
						_ => {}
					}
				}
			}
			_ => {
				if let Some(last) = last_action {
					match last {
						Actions::ReadingOpenTagName => {
							if letter == ' ' {
								parsed = create_element(reading.clone());
								reading = String::new();
								actions.pop();

								actions.push(Actions::ReadingAttributes);
							} else {
								reading = reading + letter.to_string().as_str();
							}
						}
						Actions::ReadingChildTagName => {
							let element = parse(to_read.clone());

							if let Some(ref mut parent) = parsed {
								if let Some(child) = element {
									parent.child_nodes = vec![child];
								}
							}

							actions.pop();
						}
						Actions::ReadingCloseTagName => {

						}
						Actions::ReadingSomething => {
							reading = reading + letter.to_string().as_str();
						}
						Actions::FindingTagClose => {
							reading = reading + letter.to_string().as_str();
							actions.push(Actions::ReadingSomething);
						}
						_ => {}
					}
				} else {
					reading = reading + letter.to_string().as_str();
					actions.push(Actions::ReadingSomething);
				}
				to_read = to_read.chars().skip(1).collect();
			}
		}
	}

	parsed
}
*/

pub fn parse_shallow(html: String) -> (Vec<HtmlElement>, usize) {
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
								reading = String::new();
								parsed.push(element);
								state.pop();

								state.push(Actions::ReadingInnerTag);
							}
							Actions::ReadingCloseTagName => {
								reading = String::new();
								state.pop();
								break (parsed, idx);
							}
							_ => {}
						}

						idx += 1;
					}
					_ => {
						match last_state {
							Actions::ReadingOpenTagName => {
								if letter == ' ' {
									state.pop();
									state.push(Actions::ReadingAttributes);
									parsed.push(HtmlElement::new(reading.clone()));
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
	use crate::parse;
	#[test]
	fn parse_simple_tag() {
		let html = String::from("<h1>Olá Marcos</h1>");
		let lexer = parse(html);
	}
}
