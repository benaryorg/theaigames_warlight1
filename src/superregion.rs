use std::str::FromStr;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseSuperRegionError;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct SuperRegion
{
	id: usize,
	bonus: usize,
}

impl FromStr for SuperRegion
{
	type Err = ParseSuperRegionError;

	fn from_str(s: &str) -> Result<Self,Self::Err>
	{
		let mut sp = s.split_whitespace();
		Ok(SuperRegion
		{
			id: sp.next().unwrap().parse().unwrap(),
			bonus: sp.next().unwrap().parse().unwrap(),
		})
	}
}

