#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	None,
	Float(f64),
	String(String),
	Int(isize),
}


pub fn turn_to_value(s : &str) -> Value {
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

pub fn value_as_string(v : &Value) -> String {
	match v {
		Value::None => String::new(),
		Value::String(s) => s.clone(),
		Value::Int(s) => format!("{:?}", s),
		Value::Float(s) => format!("{:?}", s),
	}
}

#[test]
fn test_value_as_string() {
	let test_cases : Vec<String>= vec!{
		"",
		"42",
		"hello world"
	}.iter().map(|x| x.to_string()).collect();

	for case in test_cases.iter() {
		assert_eq!(&value_as_string(&turn_to_value(&case)), case);
	}
}

pub fn value_to_float(v : &Value) -> Option<f64> {
	match v {
		Value::Float(f) => Some(*f),
		Value::Int(i) => Some(*i as f64),
		_ => None,
	}
}

pub fn value_to_int(v : &Value) -> Option<isize> {
	match v {
		Value::Int(n) => Some(*n),
		_ => None,
	}
}

pub fn value_to_string(v : &Value) -> Option<String> {
	match v {
		Value::String(s) => Some(s.clone()),
		_ => None,
	}
}