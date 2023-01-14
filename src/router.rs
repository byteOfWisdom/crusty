use crate::s_exp_parser::SExpr;

#[allow(dead_code)]
pub fn route(_ : KicadPcb) -> KicadPcb {
	unimplemented!();
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct KicadPcb {
	epxrs : Vec<SExpr>,
}


#[allow(dead_code)]
impl KicadPcb {
	pub fn new(_expr : SExpr) -> Self {
		KicadPcb {
			epxrs : Vec::new(),
		}
	}
}


#[test]
fn test_pcb_load() {
	unimplemented!();
}