#[allow(dead_code)]
pub fn route(_ : &KicadPcb) -> () {
	unimplemented!();
}


pub fn parse (s : String) -> Option<SExpr> {
	let chunks = s.split_whitespace().map(|x| x.to_string());
	let mut leveled_values : Vec<(usize, Value)> = Vec::new();

	let mut current_level = 0;

	for chunk in chunks {
		current_level += descends(&chunk);
		current_level -= ascends(&chunk);

		leveled_values.push((current_level, turn_to_value(&chunk)));
	}

	return Some(merge_into_exp(leveled_values));
}


#[test]
fn test_parse() {
	let test_string = "(test (nesting 1 2 3.5) string)".to_string();

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



fn merge_into_exp(leveled_values : Vec<(usize, Value)>) -> SExpr {
	if leveled_values.is_empty() {
		return SExpr::new();
	}

	let base_level = leveled_values[0].0;
	let mut in_sub_exp = false;
	let mut res = SExpr::new();
	let mut sub_exp : Vec<(usize, Value)> = Vec::new();

	for (level, value) in leveled_values.iter() {
		if *level == base_level {
			res.append_value(value.clone());
			in_sub_exp = false;
		}

		if *level > base_level {
			in_sub_exp = true;
			sub_exp.push((*level, value.clone()));
		}

		if !in_sub_exp && *level == base_level {
			res.append_exp(merge_into_exp(sub_exp.clone()));
			sub_exp = Vec::new();
		}
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
		(0, Value::String("test".to_string())),
		(1, Value::String("nesting".to_string())),
		(1, Value::Int(1)),
		(1, Value::Int(2)),
		(1, Value::Float(3.5)),
		(0, Value::String("string".to_string()))
	};

	assert_eq!(test_res, merge_into_exp(test_list));
}



fn ascends(s : &str) -> usize {
	let mut res = 0;
	for c in s.trim_end().chars().rev() {
		if c.is_whitespace() {
			continue;
		}

		if c != ')' {
			return res;
		}

		res += 1;
	}

	return res;
}

#[test]
fn test_ascends() {
	assert_eq!(ascends(&"test test)"), 1);
	assert_eq!(ascends(&"test test )"), 1);
	assert_eq!(ascends(&"test test )))"), 3);
	assert_eq!(ascends(&"test test ) "), 1);
	assert_eq!(ascends(&"test test"), 0);
	assert_eq!(ascends(&"test ) test"), 0);
}


fn descends(s : &str) -> usize {
	if s.trim_start().chars().nth(0) == Some('(') {
		return 1;
	}
	return 0;
}

#[test]
fn test_descends() {
	assert_eq!(descends(&"(test test"), 1);
	assert_eq!(descends(&"( test test"), 1);
	assert_eq!(descends(&" ( test test"), 1);
	assert_eq!(descends(&"test test"), 0);
	assert_eq!(descends(&"test ( test"), 0);
}


fn turn_to_value(s : &str) -> Value {
	let trimmed = s.trim_start()
		.trim_end()
		.trim_end_matches(")")
		.trim_start_matches("(");
	

	if let Ok(maybe_int) = trimmed.parse::<isize>() {
		return Value::Int(maybe_int);
	};


	if let Ok(maybe_float) = trimmed.parse::<f64>() {
		return Value::Float(maybe_float);
	}


	if !trimmed.is_empty() {
		return Value::String(trimmed.to_string());
	}

	return Value::None;
}

#[test]
fn test_turn_to_value() {
	assert_eq!(turn_to_value(""), Value::None);
	assert_eq!(turn_to_value("()"), Value::None);
	assert_eq!(turn_to_value("("), Value::None);
	assert_eq!(turn_to_value("test"), Value::String("test".to_string()));
	assert_eq!(turn_to_value(" test "), Value::String("test".to_string()));
	assert_eq!(turn_to_value("42"), Value::Int(42));
	assert_eq!(turn_to_value(" 42 "), Value::Int(42));
	assert_eq!(turn_to_value("4.2"), Value::Float(4.2));
	assert_eq!(turn_to_value("-4.2"), Value::Float(-4.2));
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	None,
	Float(f64),
	String(String),
	Int(isize),
//	Uint(usize), // probably not good to have ambiguity when parsing
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Either<A, B> {
	This(A),
	That(B)
}


#[derive(Debug, PartialEq)]
pub struct SExpr {
	content : Vec<Either<Value, Box<SExpr>>>
}

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
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct KicadPcb {
	epxrs : Vec<SExpr>,
}

#[allow(dead_code)]
impl KicadPcb {
	pub fn new() -> Self {
		KicadPcb {
			epxrs : Vec::new(),
		}
	}
}