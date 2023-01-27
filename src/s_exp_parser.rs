use crate::value::*;

type HalfParsed = Vec<Either<Value, Delimeter>>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Delimeter {
	Open,
	Close,
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Either<A, B> {
	This(A),
	That(B)
}


type Element = Either<Value, Box<SExpr>>;


#[derive(Debug, PartialEq, Clone)]
pub struct SExpr {
	pub content : Vec<Element>
}

#[allow(dead_code)]
impl SExpr {
	pub fn new() -> Self {
		SExpr {
			content : Vec::new(),
		}
	}

	pub fn append_exp(&mut self, s_expression : SExpr) {
		self.content.push(Either::That(Box::new(s_expression)));
	}

	pub fn append_value(&mut self, value : Value) {
		self.content.push(Either::This(value));
	}

	pub fn print(&self) -> String {
		let mut res = String::new();
		for element in self.content.iter() {
			res.push_str(&match element {
				Either::This(value) => format!(" {} ", value_as_string(value)),
				Either::That(sub_exp) => format!("({})", sub_exp.print()),
			});
		}

		return res;
	}

	pub fn get_name(&self) -> String {
		if self.content.len() == 0 {
			return String::new();
		}
		return match &self.content[0] {
			Either::This(value) => value_as_string(value),
			Either::That(_) => String::new(), 
		};
	}

	pub fn get(&self, name : &str) -> Vec<SExpr>{
		if self.get_name() == name {
			return vec!{SExpr{content : self.content[1..].to_vec()}};
		} else {
			let mut res = Vec::new();
			for sub in self.iter() {
				match sub {
					Either::This(_) => {},
					Either::That(exp) => {
						res.append(&mut exp.get(name));
					},
				};
			}

			return res;
		}
	}


	// gets the first single value associated with name
	pub fn get_value(&self, name : &str) -> Option<Value> {
		let gotten = self.get(name);

		if gotten.is_empty() { return None;}

		return match &gotten[0].content[0] {
			Either::This(value) => Some(value.clone()),
			Either::That(_ ) => None,
		};

	}


	fn is_trivial(&self) -> bool {
		if self.content.len() < 2 {
			return match self.content[0] {
				Either::This(_) => false,
				Either::That(_) => true, //in case the 1 element is an expression
			};
		}

		return false;
	}


	/// returns a new s expression, with empty outer nestings trimmed away.
	/// i.e. ((((a b c)))) becomes a b c
	/// not actuall sure if this should be here or somewhere else, like router.rs
	/// as it is fairly specific to the application, but also much nicer to write as a method
	pub fn remove_trivial(&self) -> SExpr {
		if self.is_trivial() {
			let trimmed_exp = match &self.content[0] {
				Either::This(_) => panic!("SExpr.is_trivial didn't categorize correctly. this should be unreachable."),
				Either::That(inner) => *inner.clone(),
			};

			return trimmed_exp.remove_trivial();
		}

		return self.clone();
	}


	pub fn iter(&self) -> std::slice::Iter<Element> {
		return self.content.iter();
	}

	pub fn values(&self) -> Vec<Value>{
		let mut vs = Vec::new();

		for elem in self.content.iter() {
			match elem {
				Either::This(value) => vs.push(value.clone()),
				Either::That(_) => continue,
			};
		}

		return vs;
	}

	pub fn sub_expressions(&self) -> Vec<SExpr> {
		let mut vs = Vec::new();

		for elem in self.content.iter() {
			match elem {
				Either::That(exp) => vs.push(*exp.clone()),
				Either::This(_) => continue,
			};
		}

		return vs;

	}
}


#[test]
fn test_print() {
	let test_string = "(test (nesting 1 2 3.5) string)".to_string();
	let test_expr = parse(test_string.clone()).unwrap();

	assert_eq!(parse(test_expr.print()), Some(test_expr));
}



#[test]
fn test_is_trivial() {
	let test_string = "(test (nesting 1 2 3.5) string)".to_string();
	let test_expr = parse(test_string).unwrap();

	assert_eq!(test_expr.is_trivial(), true);

	//TODO: more test data
}


#[test]
fn test_remove_trivial() {
	let test_string = "(test (nesting 1 2 3.5) string)".to_string();
	let test_expr = parse(test_string).unwrap();

	let test_res = SExpr{
		content : vec!{
			Either::This(Value::String("test".to_string())),
			Either::That(Box::new(SExpr{content : vec!{
				Either::This(Value::String("nesting".to_string())),
				Either::This(Value::Int(1)),
				Either::This(Value::Int(2)),
				Either::This(Value::Float(3.5)),
			}})),
			Either::This(Value::String("string".to_string()))
		},
	};

	assert_eq!(test_expr.remove_trivial(), test_res);
}


#[test]
fn test_sexpr_get() {
	let test_string = "test (nesting 1 2 3.5) string".to_string();
	let test_expr = parse(test_string).unwrap();

	let test_res = SExpr{
		content : vec!{
			Either::This(Value::Int(1)),
			Either::This(Value::Int(2)),
			Either::This(Value::Float(3.5)),
		},
	};

	let test_res_two = vec!{
		SExpr{
			content : vec!{
				Either::This(Value::Int(1)),
				Either::This(Value::Int(2)),
				Either::This(Value::Float(3.5)),
			},
		},
		SExpr{
			content : vec!{
				Either::This(Value::Int(5)),
				Either::This(Value::Int(6)),
				Either::This(Value::Int(7)),
			},
		}

	};

	assert_eq!(test_expr.get("nesting")[0], test_res);


	let test_string_two = "test (nesting 1 2 3.5) (nesting 5 6 7) string".to_string();
	let test_expr_two = parse(test_string_two).unwrap();

	assert_eq!(test_expr_two.get("nesting"), test_res_two);

	//TODO: more test cases

}


pub fn parse (s : String) -> Option<SExpr> {
	let chunks = s.split_whitespace().map(|x| x.to_string());
	let mut leveled_values : HalfParsed = Vec::new();

	for chunk in chunks {
		let delims : HalfParsed = get_delimeter(&chunk).iter()
			.map(|x| {let res : Either<Value, Delimeter> = Either::That(*x); res})
			.collect::<HalfParsed>();

		let mut opening : HalfParsed = Vec::new();
		let mut closing : HalfParsed = Vec::new();

		for elem in delims.iter() {
			if *elem == Either::That(Delimeter::Open) {
				opening.push(elem.clone());
			} else {
				closing.push(elem.clone());
			}
		}


		leveled_values.extend(opening);
		leveled_values.push(Either::This(turn_to_value(&chunk)));
		leveled_values.extend(closing);
	}

	//println!("{:?}", leveled_values);

	return Some(merge_into_exp(leveled_values));
}


#[test]
fn test_parse() {
	let test_string = "test (nesting 1 2 3.5) string".to_string();

	let test_res = Some(SExpr{
		content : vec!{
			Either::This(Value::String("test".to_string())),
			Either::That(Box::new(SExpr{content : vec!{
				Either::This(Value::String("nesting".to_string())),
				Either::This(Value::Int(1)),
				Either::This(Value::Int(2)),
				Either::This(Value::Float(3.5)),
			}})),
			Either::This(Value::String("string".to_string()))
		},
	});
	assert_eq!(test_res, parse(test_string));
}



fn merge_into_exp(leveled_values : HalfParsed) -> SExpr {
	let mut res = SExpr::new();
	let mut i = 0;

	while i < leveled_values.len() {
		match &leveled_values[i] {
			Either::This(value) => {
				if value != &Value::None {
					res.append_value(value.clone());
				}
				i += 1;
			},

			Either::That(Delimeter::Open) => {
				let closing_brace = get_closing_delim(&leveled_values, i);
				let sub_exp = merge_into_exp(leveled_values[i + 1 .. closing_brace].to_vec());
				res.append_exp(sub_exp);
				i = closing_brace;
			},

			_ => {i += 1;},
		};
	}

	return res;
}


#[test]
fn test_merge_into_exp() {
	let test_res = SExpr{
		content : vec!{
			Either::This(Value::String("test".to_string())),
			Either::That(Box::new(SExpr{content : vec!{
				Either::This(Value::String("nesting".to_string())),
				Either::This(Value::Int(1)),
				Either::This(Value::Int(2)),
				Either::This(Value::Float(3.5)),
			}})),
			Either::This(Value::String("string".to_string()))
		},
	};

	let test_list = vec!{
		Either::This(Value::String("test".to_string())),
		Either::That(Delimeter::Open),		
		Either::This(Value::String("nesting".to_string())),
		Either::This(Value::Int(1)),
		Either::This(Value::Int(2)),
		Either::This(Value::Float(3.5)),
		Either::That(Delimeter::Close),
		Either::This(Value::String("string".to_string()))
	};

	assert_eq!(test_res, merge_into_exp(test_list));
}



/// returns the index of the delimter closing the one at opening
fn get_closing_delim(list : &HalfParsed, opening : usize) -> usize {
	let mut level = 0;
	let mut index = opening;

	while index < list.len() {
		match list[index] {
			Either::This(_) => {},
			Either::That(Delimeter::Open) => {
				level += 1;
			},
			Either::That(Delimeter::Close) => {
				level -= 1;
			},
		};

		if level == 0 {
			return index;
		}

		index += 1;
	}

	return index;
}


#[test]
fn test_get_closing_delim() {
	let test_list = vec!{
		Either::This(Value::String("test".to_string())),
		Either::That(Delimeter::Open),		
		Either::This(Value::String("nesting".to_string())),
		Either::This(Value::Int(1)),
		Either::This(Value::Int(2)),
		Either::This(Value::Float(3.5)),
		Either::That(Delimeter::Close),
		Either::This(Value::String("string".to_string()))
	};

	assert_eq!(get_closing_delim(&test_list, 1), 6);
}


fn get_delimeter(s : &str) -> Vec<Delimeter> {
	let mut res = Vec::new();

	for c in s.trim_start().chars() {
		if c.is_whitespace() { continue }

		if c != '(' { break }

		res.push(Delimeter::Open);
	}

	for c in s.trim_end().chars().rev() {
		if c.is_whitespace() { continue }

		if c != ')' { break }

		res.push(Delimeter::Close);
	}

	return res;
}

#[test]
fn test_ascends() {
	assert_eq!(get_delimeter(&"test test)"), vec!{Delimeter::Close});
	assert_eq!(get_delimeter(&"test test )"), vec!{Delimeter::Close});
	assert_eq!(get_delimeter(&"test test )))"), vec!{Delimeter::Close, Delimeter::Close, Delimeter::Close});
	assert_eq!(get_delimeter(&"test test ) "), vec!{Delimeter::Close});
	assert_eq!(get_delimeter(&"test test"), vec!{});
	assert_eq!(get_delimeter(&"test ) test"), vec!{});
}

#[test]
fn test_descends() {
	assert_eq!(get_delimeter(&"(test test"), vec!{Delimeter::Open});
	assert_eq!(get_delimeter(&"( test test"), vec!{Delimeter::Open});
	assert_eq!(get_delimeter(&" ( test test"), vec!{Delimeter::Open});
	assert_eq!(get_delimeter(&"test test"), vec!{});
	assert_eq!(get_delimeter(&"test ( test"), vec!{});
}