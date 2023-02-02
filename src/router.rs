use std::fs::read_to_string;

use crate::s_exp_parser::SExpr;
use crate::s_exp_parser;
use crate::value::*;

#[derive(Debug)]
pub enum KicadPcbError {
	IoError(std::io::Error),
	FileType,
	ParseFail,
	NoLayer(String),
	Other(String),
}

#[derive(Debug, Copy, Clone, Default)]
enum LayerType {
	#[default]
	User,
	Signal,
}


// maybe replace all name strings with hashes:
// would use less mem and be stack allocatable instead of strings, which arent
#[derive(Debug, Default, Clone)]
struct PcbNet {
	pub id : isize,
	pub name : String,
}

impl PcbNet {
	pub fn from_exp(exp : &SExpr) -> Result<Self, KicadPcbError> {
		let values = exp.values();

		let id = match match values.get(0) {
			Some(s) => value_to_int(s),
			None => return Err(KicadPcbError::Other(format!("{:?}", exp))),
		} {
			Some(v) => v,
			None => return Err(KicadPcbError::ParseFail),
		};

		let name = match match values.get(1) {
			Some(s) => value_to_string(s),
			None => return Err(KicadPcbError::ParseFail),
		} {
			Some(v) => v,
			None => return Err(KicadPcbError::ParseFail),
		};

		return Ok(PcbNet {
			id : id,
			name : name,
		});
	}
}

#[test]
fn test_pcb_net_from_exp() {
	let test_string = "(net 1 \"GND\")";
	let exp = s_exp_parser::parse(&test_string).unwrap();
	let net = PcbNet::from_exp(&exp.get("net")[0]).unwrap();

	assert_eq!(net.id, 1);
	assert_eq!(net.name, "\"GND\"");
}


type V2 = [f64; 2];


// TODO implement this
// TODO add to parser
#[derive(Debug)]
struct Setup {
}

#[derive(Debug, Clone, Default)]
struct PcbLayer {
	pub id : usize,
	pub name : String,
	pub layer_type : LayerType,
	pub attrib : String,
}


impl PcbLayer {
	pub fn from_exp(exp : &SExpr) -> Result<Self, KicadPcbError> {
		let get_err = KicadPcbError::NoLayer(exp.print());

		if exp.values().len() == 0 {
			return Err(get_err);
		}

		let id = match value_to_int(&exp.values()[0]) {
			Some(v) => v, 
			None => return Err(get_err),
		};

		let name = match value_to_string(&exp.values()[1]) {
			Some(v) => v, 
			None => return Err(get_err),
		};

		let layer_type = match value_to_string(&exp.values()[2]) {
			Some(v) => LayerType::default(),
			None => return Err(get_err),
		};

		let attrib = match exp.values().get(3) {
			Some(value) => value_to_string(&value).unwrap(),
			None => String::new(),
		};


		return Ok(PcbLayer {
			id : id as usize,
			name : name,
			layer_type : layer_type,
			attrib : String::new(),
		});
	}
}


#[allow(dead_code)]
#[derive(Debug, Default)]
struct PcbGeneral {
	pub thickness: f64,
}


#[allow(dead_code)]
#[derive(Debug, Default)]
struct Pad {
	pub layer : Vec<String>,
	pub at : V2,
	pub net : PcbNet,
	//may need more fields
}

impl Pad {
	pub fn from_exp(exp : &SExpr) -> Result<Self, KicadPcbError> {
		let get_err = KicadPcbError::ParseFail;
		let mut pad = Pad::default();

		pad.layer = match exp.get("layers")
			.get(0) {
				Some(s) => s,
				None => return Err(get_err),
			}
			.values()
			.iter()
			.map(|x| value_to_string(&x).unwrap()) //maybe replace unwrap with a match
			.collect();

		pad.at = match exp.get("at")
			.get(0) {
				Some(s) => s,
				None => return Err(get_err),				
			}.values()
			.iter()
			.map(|x| match value_to_float(&x) {
				Some(v) => v,
				None => panic!("{:?}", x),
			}) // maybe make this a match
			.collect::<Vec<f64>>()
			.try_into()
			.unwrap(); //maybe make this a match

		return Ok(pad);
	}
}

#[test]
fn test_pad_from_exp() {
	let test_string = "(pad \"1\" smd roundrect (at -1.4 0) (size 1.25 2.65) (layers \"F.Cu\" \"F.Paste\" \"F.Mask\") (roundrect_rratio 0.2)
      (net 1 \"GND\") (pinfunction \"K\") (pintype \"passive\") (tstamp 2b94d621-c132-4657-b654-d69cf5549fbe))";
	
	let exp = s_exp_parser::parse(&test_string).unwrap();
	let pad = Pad::from_exp(&exp.get("pad")[0]).unwrap();

	assert_eq!(pad.at, [-1.4, 0.0]);
}



#[allow(dead_code)]
#[derive(Debug, Default)]
struct Footprint {
	pub name : String,
	pub layer : String,
	pub at : V2,
	pub pads : Vec<Pad>,
	//may need more fields
}


impl Footprint {
	pub fn from_exp(exp : &SExpr) -> Result<Self, KicadPcbError> {
		let get_err = KicadPcbError::ParseFail;

		let mut footprint = Footprint::default();

		footprint.name = exp.get_name();


		footprint.pads = exp.get("pad").iter()
			.filter_map(|x| match Pad::from_exp(&x) {
				Ok(v) => Some(v), 
				Err(e) => None
			})
			.collect();

		footprint.layer = match value_to_string(
			& match exp.get("layer").get(0) {
				Some(l) => l,
				None => return Err(get_err),
			}.values()[0]
		){
			Some(l) => l,
			None => return Err(get_err),
		};

		footprint.at = match exp.get("at")
			.get(0) {
				Some(s) => s,
				None => return Err(get_err),				
			}.values()
			.iter()
			.map(|x| match value_to_float(&x) {
				Some(v) => v,
				None => panic!("{:?}", x),
			}) // maybe make this a match
			.collect::<Vec<f64>>()
			.try_into()
			.unwrap(); //maybe make this a match

		return Ok(footprint);
	}
}


#[allow(dead_code)]
#[derive(Debug)]
//only contains information relevant for routing, not a complete representation
pub struct KicadPcb {
	general : 	PcbGeneral,
	layers : Vec<PcbLayer>,
	nets : Vec<PcbNet>,
	footprints : Vec<Footprint>
}


#[allow(dead_code)]
impl KicadPcb {
	pub fn new(_expr : SExpr) -> Self {
		KicadPcb {
			general : PcbGeneral{ thickness: 0.0},
			layers : Vec::new(),
			nets : Vec::new(),
			footprints : Vec::new(),
		}
	}


	pub fn from_file(file : &str) -> Result<Self, KicadPcbError> {
		if !file.ends_with(&".kicad_pcb") {
			return Err(KicadPcbError::FileType);
		}

		let data = match read_to_string(file) {
			Ok(d) => d,
			Err(e) => return Err(KicadPcbError::IoError(e)),
		};

		let epxrs = match s_exp_parser::parse(&data) {
			Some(exp) => exp,
			None => return Err(KicadPcbError::ParseFail),
		};

		let pcb_exp = epxrs.remove_trivial();

		
		// get all the relevant parts from the expression
		//--------------------------------------------------
		let pcb = KicadPcb{
			general : match get_general(&pcb_exp) {
				Ok(result) => result,
				Err(e) => return Err(e),
			},

			layers : match get_layers(&pcb_exp) {
				Ok(result) => result,
				Err(e) => return Err(e),
			},
			
			nets : match get_nets(&pcb_exp) {
				Ok(result) => result,
				Err(e) => return Err(e),
			},
			
			footprints : match get_footprints(&pcb_exp) {
				Ok(result) => result,
				Err(e) => return Err(e),
			},
		};

		return Ok(pcb);
	}

	pub fn write_to_file(path : &str) {
		unimplemented!();
	}


	fn as_s_expr(&self) -> SExpr {
		unimplemented!();
	}
}

#[allow(dead_code)]
pub fn route(_ : KicadPcb) -> KicadPcb {
	unimplemented!();
}


fn get_general(exp : &SExpr) -> Result<PcbGeneral, KicadPcbError> {
	let get_err = Err(KicadPcbError::ParseFail);

	for general_section in exp.get("general").iter() {
		match general_section.get_value("thickness") {
			Some(Value::Float(f)) => {
				return Ok(PcbGeneral{thickness : f});
			},
			_ => continue,
		};
	}

	return get_err;
}

#[test]
fn test_get_general() {
	let test_pcb_general = get_general(
		&s_exp_parser::parse(
			&read_to_string("./test_pcb/test_pcb.kicad_pcb").unwrap()
		).unwrap()
	).unwrap();

	assert_eq!(test_pcb_general.thickness, 1.6);
}


fn get_layers(exp : &SExpr) -> Result<Vec<PcbLayer>, KicadPcbError> {
	let mut layers = Vec::new();


	// assumes the first "layers" tag to be the declaration
	for layer in exp.get("layers")[0].sub_expressions().iter() {
		println!("{:?}\n", &layer);
		match PcbLayer::from_exp(layer) {
			Ok(l) => layers.push(l),
			Err(_) => continue,
		};
	}

	return Ok(layers);
}

#[test]
fn test_get_layers() {
	let test_pcb = &s_exp_parser::parse(
		&read_to_string("./test_pcb/test_pcb.kicad_pcb").unwrap()
	).unwrap();

	let layers = get_layers(test_pcb).unwrap();

	assert_eq!(layers.len(), 29);
}


fn get_nets(exp : &SExpr) -> Result<Vec<PcbNet>, KicadPcbError> {
	let get_err = Err(KicadPcbError::ParseFail);


	return get_err;
}


fn get_footprints(exp : &SExpr) -> Result<Vec<Footprint>, KicadPcbError> {
	let raw_footprints = exp.get("footprint");

	let mut done_footprints = Vec::new();

	for footprint in raw_footprints.iter() {
		match Footprint::from_exp(footprint) {
			Ok(f) => {done_footprints.push(f)},
			Err(e) => return Err(e),
		};
	}

	return Ok(done_footprints);
}



#[test]
fn test_get_footprints() {
	let test_pcb = &s_exp_parser::parse(
		&read_to_string("./test_pcb/test_pcb.kicad_pcb").unwrap()
	).unwrap();

	let footprints = get_footprints(&test_pcb).unwrap();

	//panic!("{:?}", footprints);

	assert_eq!(footprints.len(), 2);
}


#[test]
fn test_pcb_load() {
	unimplemented!();
}