use player::Player;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum Turn
{
	Place
	{
		name: Player,
		region: usize,
		count: usize,
	},
	Turn
	{
		name: Player,
		source: usize,
		target: usize,
		count: usize,
	},
}

