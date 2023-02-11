use std::fs::read_to_string;

use crate::s_exp_parser::SExpr;
use crate::s_exp_parser;
use crate::value::*;


pub type NetId = usize;
pub type LayerId = usize;
pub type V2 = [f64; 2];


#[derive(Debug)]
pub enum KicadPcbError {
	IoError(std::io::Error),
	FileType,
	ParseFail,
	PcbNetFail,
	GeneralFail,
	FootprintFail,
	PadFail,
	WireFail,
	ViaFail,
	NoLayer(String),
	Other(String),
}


#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum LayerType {
	#[default]
	User,
	Signal,
}


#[derive(Debug, Default, Clone)]
pub struct Wire {
	pub net_id : NetId,
	pub layer_name : String,
	pub start : V2,
	pub end : V2,
}


impl Wire {
	pub fn from_exp(exp : &SExpr) -> Result<Self, KicadPcbError> {
		let get_err = KicadPcbError::WireFail;

		let start = match exp.get("start")
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

		let end = match exp.get("end")
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

		let net_id = value_to_int(&exp.get("net")[0].values()[0]).unwrap() as usize;

		let layer_name = value_to_string(&exp.get("layer")[0].values()[0]).unwrap();

		return Ok(Wire{
			net_id : net_id,
			layer_name : layer_name,
			start : start,
			end : end,
		});
	}
}


#[derive(Debug, Default, Clone)]
pub struct Via {
	pub net_id : NetId,
	pub at : V2,
	pub layers : Vec<String>,
}

impl Via {
	pub fn from_exp(exp : &SExpr) -> Result<Self, KicadPcbError> {
		let get_err = KicadPcbError::ViaFail;

		let at = match exp.get("at")
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


		let layers = exp.get("layers")[0]
			.values()
			.iter()
			.filter_map(value_to_string)
			.collect();

		let net_id = value_to_int(
			&exp.get("net")[0].values()[0]
		).unwrap() as usize;

		return Ok(Via{
			at : at,
			layers : layers,
			net_id : net_id,
		});

	}
}


// maybe replace all name strings with hashes:
// would use less mem and be stack allocatable instead of strings, which arent
#[derive(Debug, Default, Clone)]
pub struct PcbNet {
	pub id : NetId,
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
			id : id as usize,
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


// TODO implement this
// TODO add to parser
#[derive(Debug)]
pub struct Setup {
}

#[derive(Debug, Clone, Default)]
pub struct PcbLayer {
	pub id : LayerId,
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
			Some(_) => LayerType::default(),
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
			attrib : attrib,
		});
	}
}


#[derive(Debug, Default)]
pub struct PcbGeneral {
	pub thickness: f64,
}


#[derive(Debug, Default, Clone)]
pub struct Pad {
	pub layer : Vec<String>,
	pub at : V2,
	pub abs_at : V2,
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

		pad.abs_at = pad.at;

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



#[derive(Debug, Default)]
pub struct Footprint {
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
				_ => None
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

		// write the absolute positions of the pads
		footprint.pads = footprint.pads
			.iter()
			.map(|p| {
				let mut r = p.clone();
				r.abs_at[0] += footprint.at[0];
				r.abs_at[1] += footprint.at[1];
				return r;
			}).collect();

		return Ok(footprint);
	}
}


#[derive(Debug)]
//only contains information relevant for routing, not a complete representation
pub struct KicadPcb {
	pub general : PcbGeneral,
	pub layers : Vec<PcbLayer>,
	pub nets : Vec<PcbNet>,
	pub footprints : Vec<Footprint>,
	pub wires : Vec<Wire>,
	pub vias : Vec<Via>,
}


impl KicadPcb {
	pub fn new(_expr : SExpr) -> Self {
		KicadPcb {
			general : PcbGeneral{ thickness: 0.0},
			layers : Vec::new(),
			nets : Vec::new(),
			footprints : Vec::new(),
			wires : Vec::new(),
			vias : Vec::new(),
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

			wires : match get_wires(&pcb_exp) {
				Ok(result) => result,
				Err(e) => return Err(e),
			},

			vias : match get_vias(&pcb_exp) {
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

	pub fn get_layer_id(&self, name : &str) -> Option<LayerId> {
		self.layers.iter()
			.filter_map(
				|x| {if x.name == name {return Some(x.id); } return None;}
			).next()
	}

	pub fn routable_layers(&self) -> usize {
		return self.layers
			.iter()
			.filter_map(|x| match x.layer_type {
				LayerType::Signal => Some(()),
				_ => None,
			})
			.count();
	}


	pub fn route(&self, settings : &RouterSettings) -> Option<KicadPcb> {
		// convert into abstract route graph

		//route it

		//convert back from absctract route graph

		return None;
	}
}


#[derive(Debug, Copy, Clone, Default)]
pub struct RouterSettings {
	pub max_passes : usize,
	pub algorith : u8,
}



#[test]
fn test_route() {
	let test_pcb = KicadPcb::from_file("./test_pcb/test_pcb.kicad_pcb").unwrap();
	let settings = RouterSettings::default();
	test_pcb.route(&settings).unwrap();
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
		.map(PcbLayer::from_exp)
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
		.filter_map(|x| match PcbNet::from_exp(x) {
			Ok(v) => Some(v),
			_ => None,
		})
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
		.map(Footprint::from_exp)
		.collect()
}


#[test]
fn test_get_footprints() {
	let test_pcb = &s_exp_parser::parse(
		&read_to_string("./test_pcb/test_pcb.kicad_pcb").unwrap()
	).unwrap();

	let footprints = get_footprints(&test_pcb).unwrap();

	//panic!("{:?}", footprints);

	assert_eq!(footprints.len(), 4);
}


fn get_wires(exp : &SExpr) -> Result<Vec<Wire>, KicadPcbError> {
	exp.get("segment")
		.iter()
		.map(Wire::from_exp)
		.collect()
}


#[test]
fn test_get_wires() {
	let test_pcb = &s_exp_parser::parse(
		&read_to_string("./test_pcb/test_pcb.kicad_pcb").unwrap()
	).unwrap();

	let wires = get_wires(&test_pcb).unwrap();

	assert_eq!(wires.len(), 2);
}


fn get_vias(exp : &SExpr) -> Result<Vec<Via>, KicadPcbError> {
	exp.get("via")
		.iter()
		.map(Via::from_exp)
		.collect()	
}

#[test]
fn test_get_vias() {
	let test_pcb = &s_exp_parser::parse(
		&read_to_string("./test_pcb/test_pcb.kicad_pcb").unwrap()
	).unwrap();

	let vias = get_vias(&test_pcb).unwrap();

	assert_eq!(vias.len(), 1);

}

#[test]
fn test_pcb_load() {
	let _test_pcb = KicadPcb::from_file("./test_pcb/test_pcb.kicad_pcb").unwrap();
	//TODO: how do i even write a test for this??
}