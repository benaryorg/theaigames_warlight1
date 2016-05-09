use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseSuperRegionError;

impl From<ParseIntError> for ParseSuperRegionError
{
	fn from(_: ParseIntError) -> ParseSuperRegionError
	{
		ParseSuperRegionError
	}
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct SuperRegion
{
	pub id: usize,
	pub bonus: usize,
}

impl FromStr for SuperRegion
{
	type Err = ParseSuperRegionError;

	fn from_str(s: &str) -> Result<Self,Self::Err>
	{
		let mut sp = s.split_whitespace();
		Ok(SuperRegion
		{
			id: try!(sp.next().unwrap().parse()),
			bonus: try!(sp.next().unwrap().parse()),
		})
	}
}

