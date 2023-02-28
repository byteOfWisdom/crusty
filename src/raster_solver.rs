use crate::router::KicadPcb;
use crate::router::V2;

#[derive(Debug, Clone, Copy, Default)]
struct Discrete3D {
	pub x : usize,
	pub y : usize,
	pub layer : usize,
}

impl Discrete3D {
	pub fn from(x : usize, y : usize, layer : usize) -> Self {
		return Self{
			x : x,
			y : y,
			layer : layer,
		}
	}
}


#[derive(Debug, Default, Clone)]
struct Raster {
	data : Vec<GridState>,
	x_cells : usize,
	y_cells : usize,
	layers : usize,
	spacing : f64,
}


impl Raster {
	pub fn new(board_params : &KicadPcb) -> Self {
		//decide grid spacing
		let spacing : f64 = 0.1; //no!!

		let x_size : f64 = 10.0; //pcb needs to contain this info
		let y_size : f64= 10.0;

		//init grid
		let x = (x_size / spacing).ceil() as usize;
		let y = (y_size / spacing).ceil() as usize;
		let z = board_params.routable_layers();

		let mut raster = Self{
			data : vec![GridState::Free ; x * y * z],
			x_cells : x,
			y_cells : y,
			layers : z,
			spacing : spacing,
		};


		//put pads and vias and existing wires down
		for pad in board_params
			.footprints
			.iter()
			.flat_map(|x| x.pads.iter()) 
		{
			//TODO: place pads of actual size instead of one single cell
			let point = raster.get_discrete(pad.abs_at, 1);
			raster.set(point, GridState::Pad);
		}

		for via in board_params.vias.iter() {
			for layer in via.layers.iter() {
				let point = raster.get_discrete(via.at, board_params.get_layer_id(&layer).unwrap());
				raster.set(point, GridState::Via);
			}
		}

		for wire in board_params.wires.iter() {
			todo!();
		}


		return Self::default();
	}

	pub fn get(&self, pos : Discrete3D) -> GridState {
		return *self.data.get(self.index(pos)).unwrap();
	}

	pub fn set(&mut self, pos : Discrete3D, value : GridState) {
		let index = self.index(pos);
		self.data[index] = value;
	}

	fn index(&self, point : Discrete3D) -> usize {
		return self.layers * point.layer + self.x_cells * point.x + point.y;
	}

	fn get_discrete(&self, at : V2, layer : usize) -> Discrete3D {
		let x = (at[0] / self.spacing) as usize;
		let y = (at[1] / self.spacing) as usize;
		return Discrete3D::from(x, y, layer);
	}

	pub fn route(&mut self) {
		//http://www.eecs.northwestern.edu/~haizhou/357/lec6.pdf
		todo!();
	}
}


#[derive(
	Debug, 
	Default, 
	Copy, 
	Clone, 
	PartialEq
)]
enum GridState {
	#[default]
	Free,
	Pad,
	Wire,
	Via,
	UserWire, //so they cant get removed
	UserVia,
}


#[test]
fn test_raster_gen() {
	let test_pcb = KicadPcb::from_file("./test_pcb/test_pcb.kicad_pcb").unwrap();
	let mut raster = Raster::new(&test_pcb);
}

#[test]
fn test_raster_get_set() {
	unimplemented!();
}

#[test]
fn test_raster_get_discrete() {
	unimplemented!();
}

#[test]
fn test_raster_get_route() {
	unimplemented!();
}