use std::str::FromStr;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseRegionError;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Region
{
	pub id: usize,
	pub count: usize,
	pub super_region: usize,
	pub player: String,
	pub neighbours: Vec<usize>,
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
			player: "unknown".into(),
			neighbours: Vec::new(),
		})
	}
}

