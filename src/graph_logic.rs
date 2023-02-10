#[derive(Debug, Default, Clone)]
struct RouteGraph {
	nodes : Vec<Node>,
	connections : Vec<Connection>,
}


#[derive(Debug, Default, Clone, Copy)]
struct Node {
	id : usize,
	layer : usize,
}

#[derive(Debug, Clone, Copy)]
struct Connection {
	a : usize,
	b : usize,
}