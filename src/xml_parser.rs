
#[derive(Debug, Clone)]
struct Node {
	name: String,
	child: Vec<Node>,
	value: String,
}

impl Node {
	fn new_child(name: String, child: Vec<Node>) -> Node {
		Node {
			name: name,
			child: child,
			value: String::new(),
		}
	}
	fn new_value(name: String, value: String) -> Node {
		Node {
			name: name,
			child: Vec::new(),
			value: value,
		}
	}

	fn has_children(&self) -> bool {
		!self.child.is_empty()
	}

	fn has_value(&self) -> bool {
		!self.value.is_empty()
	}
	
	fn has_child_with_name(&self, name: &String) -> bool {
		for child in &self.child {
			if child.name == *name {
				return true;
			}
		}
		false
	}

	fn get_value(&self) -> &String {
		&self.value
	}

	fn get_child_with_name(&self, name: &String) -> Option<Vec<&Node>> {
		let mut result: Vec<&Node> = Vec::new();
		for child in &self.child {
			println!("{}: {}", child.name, *name);
			if child.name == *name {
				println!("Found");
				result.push(child);
			}
		}
		if !result.is_empty() {
			return Some(result);
		}
		None
	}
}

#[derive(Debug, Clone)]
pub struct XMLParser {
	root: Node,
}

impl XMLParser {
	pub fn parse(xml: String) -> XMLParser {
		let mut tags: Stack<String> = Stack::new();
		let mut in_tag = false;
		let mut current_open_tag = String::new();
		let mut current_close_tag = String::new();
		let mut tag_contents = String::new();
		let mut check_close_tag = false;
		let mut is_closing_tag = false;
		let mut active_node: Vec<Node> = Vec::new();
		let mut prev_node: Vec<Node> = Vec::new();
		for line in xml.lines() {
			for letter in line.chars() {
				if letter == '>' {
					if !is_closing_tag {
						tags.push(current_open_tag.clone());
					} else {
						tags.pop();
					}
					in_tag = false;
					is_closing_tag = false;
				} else if in_tag {
					if check_close_tag && letter == '/' {
						check_close_tag = false;
						is_closing_tag = true;
					} else if is_closing_tag{
						current_close_tag.push(letter);
					} else {
						current_open_tag.push(letter);
					}
				} else if letter == '<' {
					in_tag = true;
					check_close_tag = true;
				} else {
					// println!("{}", letter);
					if letter != ' ' {
						tag_contents.push(letter);
					}
					// println!("{}", tag_contents);
				}
			}
			if tag_contents == "" {
				if current_open_tag != "" {
					// println!("Open {}", current_open_tag); // Opening tag
					// let node = Node::new(current_open_tag.clone(), NodeType::none);
					// active_node.push(node);
					prev_node = active_node;
					active_node = Vec::new();
				} else {
					// println!("Close {}", current_close_tag); // Closing tag
					// println!("{:?}", active_node);
					let node = Node::new_child(current_close_tag, active_node);
					// println!("{:?}", node);
					active_node = prev_node;
					prev_node = Vec::new();
					active_node.push(node);
				}
			} else {
				// println!("{}: {} ({})", current_open_tag, tag_contents, tags.peek().unwrap() ); // Tag with value
				let node = Node::new_value(current_open_tag, tag_contents);
				active_node.push(node);
			}
			tag_contents = String::new();
			current_open_tag = String::new();
			current_close_tag = String::new();
			// println!("{:?}", tags);
		}
		// println!("\n\n{:?}", active_node);
		XMLParser {
			// root: Node::new(current_open_tag, NodeType::value(tag_contents)),
			root: active_node[0].clone(),
		}
	}

	/*pub fn get_value(&self, mut path: Vec<String>) -> Option<&String> {
		if path[0] != self.root.name {
			return None;
		}
		path.remove(0);
		let mut node = &self.root;
		for tag in path {
			if node.has_child_with_name(&tag) {
				node = node.get_child_with_name(&tag).unwrap();
			} else {
				return None;
			}
		}
		if node.has_value() {
			Some(node.get_value())
		} else {
			None
		}
	} */

	pub fn get_value(&self, mut path: Vec<&str>) -> Vec<Option<String>> {
		if path[0] != self.root.name {
			return vec!(None);
		}
		path.remove(0);
		self.get_value_recursive(&self.root.child, path)
	}

	fn get_value_recursive(&self, nodes: &Vec<Node>,path: Vec<&str>) ->  Vec<Option<String>> {
		let mut result: Vec<Option<String>> = Vec::new();
		let mut n = 0;
		// let nodes = nodes.clone();
		for node in nodes {
			if node.name == path[0] {
				if path.len() == 1 {
					if node.has_value() {
						result.push(Some(nodes[n].get_value().clone()));
					} /*else {
						return vec!(None);
					}*/
				} else {
					let mut new_path = path.clone();
					new_path.remove(0);
					for i in self.get_value_recursive(&node.child, new_path) {
						result.push(i);
					}
				}
			}
			n += 1;
		}
		if result.len() == 0 {
			result.push(None);
		}
		result
	}

}

#[derive(Debug)]
struct Stack<T> {
  stack: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Stack { stack: Vec::new() }
  }

  fn length(&self) -> usize {
    self.stack.len()
  }

  fn pop(&mut self) -> Option<T> {
    self.stack.pop()
  }

  fn push(&mut self, item: T) {
    self.stack.push(item)
  }

  fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }

  fn peek(&self) -> Option<&T> {
    self.stack.last()
  }
}