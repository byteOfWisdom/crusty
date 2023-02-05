use std::fs::read_to_string;

use crate::s_exp_parser::SExpr;
use crate::s_exp_parser;
use crate::value::*;

#[derive(Debug)]
pub enum KicadPcbError {
	IoError(std::io::Error),
	FileType,
	ParseFail,
	PcbNetFail,
	GeneralFail,
	FootprintFail,
	PadFail,
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

impl PartialEq for PcbNet {
	fn eq(&self, other : &Self) -> bool {
		self.id == other.id && self.name == other.name
	}

	fn ne(&self, other : &Self) -> bool {
		! (self == other)
	}

}

impl PcbNet {
	pub fn from_exp(exp : &SExpr) -> Result<Self, KicadPcbError> {
		let values = exp.values();

		let id = match match values.get(0) {
			Some(s) => value_to_int(s),
			None => return Err(KicadPcbError::Other(format!("{:?}", exp))),
		} {
			Some(v) => v,
			None => return Err(KicadPcbError::PcbNetFail),
		};

		let name = match match values.get(1) {
			Some(s) => value_to_string(s),
			None => return Err(KicadPcbError::PcbNetFail),
		} {
			Some(v) => v,
			None => return Err(KicadPcbError::PcbNetFail),
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
		let get_err = KicadPcbError::PadFail;
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
		let get_err = KicadPcbError::FootprintFail;

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


	#[allow(dead_code)]
	pub fn route(&self, settings : RouterSettings) -> Option<KicadPcb> {
		return None;
	}
}


#[derive(Debug, Copy, Clone, Default)]
struct RouterSettings {
	pub max_passes : usize,
}



#[test]
fn test_route() {
	let test_pcb = KicadPcb::from_file("./test_pcb/test_pcb.kicad_pcb").unwrap();
	let settings = RouterSettings::default();
	test_pcb.route(settings).unwrap();
}



fn get_general(exp : &SExpr) -> Result<PcbGeneral, KicadPcbError> {
	match exp.get("general")
		.iter()
		.filter_map(|x| 
			match x.get_value("thickness") {
				Some(Value::Float(f)) => Some(PcbGeneral{thickness : f}),
				_ => None,
		} )
		.next() 
	{
		Some(v) => Ok(v),
		None => Err(KicadPcbError::GeneralFail)
	}
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
	exp.get("layers")[0]
		.sub_expressions()
		.iter()
		.map(| elem | PcbLayer::from_exp(elem))
		.collect()
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
	let mut nets : Vec<PcbNet> = Vec::new();

	// any mention of a net can be treated as a declaration as such
	// there is no need to distinguish between declaration and other mentions
	for net in exp
		.get("net")
		.iter()
		.map(|x| PcbNet::from_exp(x).unwrap())
	{
		if !nets.contains(&net) {
			nets.push(net.clone());
		}
	}

	return Ok(nets);
}

#[test]
fn test_get_nets() {
	let test_pcb = &s_exp_parser::parse(
		&read_to_string("./test_pcb/test_pcb.kicad_pcb").unwrap()
	).unwrap();

	let nets = get_nets(test_pcb).unwrap();

	assert_eq!(nets.len(), 4);

}


fn get_footprints(exp : &SExpr) -> Result<Vec<Footprint>, KicadPcbError> {
	exp.get("footprint")
		.iter()
		.map(| elem | Footprint::from_exp(elem))
		.collect()
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


fn get_wires(exp : &SExpr) -> () {
	todo!();
}


#[test]
fn test_get_wires() {
	unimplemented!();
}


fn get_vias(exp : &SExpr) -> () {
	todo!();
}

#[test]
fn test_get_vias() {
	unimplemented!();
}

#[test]
fn test_pcb_load() {
	let test_pcb = KicadPcb::from_file("./test_pcb/test_pcb.kicad_pcb").unwrap();
	//TODO: how do i even write a test for this??
}