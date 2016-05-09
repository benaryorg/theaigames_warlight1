use std::collections::BTreeSet;
use std::num::ParseIntError;
use std::str::FromStr;

use player::Player;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseRegionError;

impl From<ParseIntError> for ParseRegionError
{
	fn from(_: ParseIntError) -> ParseRegionError
	{
		ParseRegionError
	}
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Region
{
	pub id: usize,
	pub count: usize,
	pub super_region: usize,
	pub player: Player,
	pub neighbours: BTreeSet<usize>,
}

impl FromStr for Region
{
	type Err = ParseRegionError;

	fn from_str(s: &str) -> Result<Self,Self::Err>
	{
		let mut sp = s.split_whitespace();
		Ok(Region
		{
			id: try!(sp.next().unwrap().parse()),
			count: 0,
			super_region: try!(sp.next().unwrap().parse()),
			player: Player::Unknown,
			neighbours: BTreeSet::new(),
		})
	}
}

