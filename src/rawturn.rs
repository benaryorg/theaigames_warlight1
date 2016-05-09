use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseRawTurnError;

impl From<ParseIntError> for ParseRawTurnError
{
	fn from(_: ParseIntError) -> ParseRawTurnError
	{
		ParseRawTurnError
	}
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum RawTurn
{
	StartingRegions(usize,usize,usize,usize,usize,usize),
	Place
	{
		name: String,
		region: usize,
		count: usize,
	},
	Turn
	{
		name: String,
		source: usize,
		target: usize,
		count: usize,
	},
	Noop,
}

impl FromStr for RawTurn
{
	type Err = ParseRawTurnError;

	fn from_str(s: &str) -> Result<Self,Self::Err>
	{
		use self::RawTurn::*;

		let args = s.split_whitespace().collect::<Vec<_>>();
		match args.len()
		{
			2 => Ok(Noop),
			4 => Ok(
				Place
				{
					name: args[0].to_owned(),
					region: try!(args[2].parse()),
					count: try!(args[3].parse()),
				}),
			5 => Ok(
				Turn
				{
					name: args[0].to_owned(),
					source: try!(args[2].parse()),
					target: try!(args[3].parse()),
					count: try!(args[4].parse()),
				}),
			6 =>
			{
				let args = try!(args.iter().map(|s|s.parse::<usize>()).collect::<Result<Vec<usize>,ParseIntError>>());
				Ok(StartingRegions
				(
					args[0],
					args[1],
					args[2],
					args[3],
					args[4],
					args[5],
				))
			},
			_ => Err(ParseRawTurnError),
		}
	}
}

impl fmt::Display for RawTurn
{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result
	{
		use self::RawTurn::*;
		match self
		{
			&StartingRegions(a,b,c,d,e,f_) => write!(f,"{} {} {} {} {} {}",a,b,c,d,e,f_),
			&Place{ref name,region,count} => write!(f,"{} place_armies {} {}",name,region,count),
			&Turn{ref name,source,target,count} => write!(f,"{} attack/transfer {} {} {}",name,source,target,count),
			&Noop => write!(f,"No moves"),
		}
	}
}

