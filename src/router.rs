use crate::s_exp_parser::SExpr;
use crate::s_exp_parser;

use std::fs::read_to_string;
use std::collections::HashMap;


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


type TypeMap = HashMap<&'static str, PartType>;


// maybe replace all name strings with hashes:
// would use less mem and be stack allocatable instead of strings, which arent
type PcbGeneral = ();
type PcbNet = (usize, String);
type PcbLayer = (usize, String, String); // no!
type V2 = [f64; 2];


#[allow(dead_code)]
#[derive(Debug)]
struct Pad {
	layer : PcbLayer,
	at : V2,
	net : PcbNet,
	//may need more fields
}


#[allow(dead_code)]
#[derive(Debug)]
struct Footprint {
	name : String,
	layer : PcbLayer,
	at : V2,
	pads : Vec<Pad>,
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


#[derive(Debug)]
enum PartType {
	General,
	Net,
	Layer,
	Footprint,
}


fn mak_key_map() -> TypeMap{
	HashMap::from([
		("general", 	PartType::General),
		("net", 		PartType::Net),
		("layer", 		PartType::Layer),
		("footprint", 	PartType::Footprint),
	])
}


#[allow(dead_code)]
impl KicadPcb {
	pub fn new(_expr : SExpr) -> Self {
		KicadPcb {
			general : (),
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

		let type_map = mak_key_map();


		return Err(KicadPcbError::ParseFail);
	}
}


#[test]
fn test_pcb_load() {
	unimplemented!();
}