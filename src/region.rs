use std::str::FromStr;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseRegionError;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Region
{
	id: usize,
	count: usize,
	super_region: usize,
	player: Option<String>,
	neighbours: Vec<usize>,
}

impl FromStr for Region
{
	type Err = ParseRegionError;

	fn from_str(s: &str) -> Result<Self,Self::Err>
	{
		let mut sp = s.split_whitespace();
		Ok(Region
		{
			id: sp.next().unwrap().parse().unwrap(),
			count: 0,
			super_region: sp.next().unwrap().parse().unwrap(),
			player: None,
			neighbours: Vec::new(),
		})
	}
}

