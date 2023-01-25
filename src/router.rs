use std::fs::read_to_string;

use crate::s_exp_parser::SExpr;
use crate::s_exp_parser;


#[allow(dead_code)]
pub fn route(_ : KicadPcb) -> KicadPcb {
	unimplemented!();
}


#[derive(Debug)]
pub enum KicadPcbError {
	IoError(std::io::Error),
	FileType,
	ParseFail,
}

// maybe replace all name strings with hashes:
// would use less mem and be stack allocatable instead of strings, which arent
type PcbNet = (usize, String);
type PcbLayer = (usize, String, String); // no!
type V2 = [f64; 2];


#[allow(dead_code)]
#[derive(Debug)]
struct PcbGeneral {
	pub thickness: f64,
}


#[allow(dead_code)]
#[derive(Debug)]
struct Pad {
	pub layer : PcbLayer,
	pub at : V2,
	pub net : PcbNet,
	//may need more fields
}


#[allow(dead_code)]
#[derive(Debug)]
struct Footprint {
	pub name : String,
	pub layer : PcbLayer,
	pub at : V2,
	pub pads : Vec<Pad>,
	//may need more fields
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

		let epxrs = match s_exp_parser::parse(data) {
			Some(exp) => exp,
			None => return Err(KicadPcbError::ParseFail),
		};

		let pcb_exp = epxrs.remove_trivial();

		
		// get all the relevant parts from the expression
		//--------------------------------------------------
		let mut pcb = KicadPcb{
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
}


fn get_general(exp : &SExpr) -> Result<PcbGeneral, KicadPcbError> {
	let get_err = Err(KicadPcbError::ParseFail);

	for expression in exp.get("general").iter() {
		println!("{:?}", expression);
	}	

	return get_err;
}

#[test]
fn test_get_general() {
	
}


fn get_layers(exp : &SExpr) -> Result<Vec<PcbLayer>, KicadPcbError> {
	let get_err = Err(KicadPcbError::ParseFail);


	return get_err;
}

fn get_nets(exp : &SExpr) -> Result<Vec<PcbNet>, KicadPcbError> {
	let get_err = Err(KicadPcbError::ParseFail);


	return get_err;
}

fn get_footprints(exp : &SExpr) -> Result<Vec<Footprint>, KicadPcbError> {
	let get_err = Err(KicadPcbError::ParseFail);


	return get_err;
}



#[test]
fn test_pcb_load() {
	unimplemented!();
}