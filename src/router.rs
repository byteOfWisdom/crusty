#[allow(dead_code)]
pub fn route(_ : &KicadPcb) -> () {
	unimplemented!();
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Delimeter {
	Open,
	Close,
	None,
}


pub fn parse (s : String) -> Option<SExpr> {
	let chunks = s.split_whitespace().map(|x| x.to_string());
	let mut leveled_values : Vec<Either<Value, Delimeter>> = Vec::new();

	for chunk in chunks {
		let delims : Vec<Either<Value, Delimeter>> = get_delimeter(&chunk).iter()
			.map(|x| {let res : Either<Value, Delimeter> = Either::That(*x); res})
			.collect::<Vec<Either<Value, Delimeter>>>();
		leveled_values.extend(delims);

		leveled_values.push(Either::This(turn_to_value(&chunk)));
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



fn merge_into_exp(leveled_values : Vec<Either<Value, Delimeter>>) -> SExpr {
	let mut res = SExpr::new();

	for elem in leveled_values.iter() {
		match elem {
			Either::This(value) => {res.append_value(value.clone())},
			Either::That(delim) => {},
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
		(0, Value::String("test".to_string())),
		(1, Value::String("nesting".to_string())),
		(1, Value::Int(1)),
		(1, Value::Int(2)),
		(1, Value::Float(3.5)),
		(0, Value::String("string".to_string()))
	};

	assert_eq!(test_res, merge_into_exp(test_list));
}


fn get_delimeter(s : &str) -> Vec<Delimeter> {
	let mut res = Vec::new();

	for c in s.trim_end().chars() {
		if c.is_whitespace() { continue }

		if c != ')' { break }

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
	assert_eq!(get_delimeter(&"test test"), vec!{Delimeter::None});
	assert_eq!(get_delimeter(&"test ) test"), vec!{Delimeter::None});
}

#[test]
fn test_descends() {
	assert_eq!(get_delimeter(&"(test test"), 1);
	assert_eq!(get_delimeter(&"( test test"), 1);
	assert_eq!(get_delimeter(&" ( test test"), 1);
	assert_eq!(get_delimeter(&"test test"), 0);
	assert_eq!(get_delimeter(&"test ( test"), 0);
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