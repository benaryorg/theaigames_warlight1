#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
struct Region
{
	id: usize,
	count: usize,
	super_region: usize,
	player: Option<String>,
	neighbours: Vec<usize>,
}

