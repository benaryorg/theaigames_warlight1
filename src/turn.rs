use std::str::FromStr;
use std::fmt;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseTurnError;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum Turn
{
	StartingRegions(usize,usize,usize,usize,usize,usize),
	Place(String,usize,usize),
	Turn(String,usize,usize,usize),
	Noop,
}

impl FromStr for Turn
{
	type Err = ParseTurnError;

	fn from_str(s: &str) -> Result<Self,Self::Err>
	{
		use self::Turn::*;

		let args = s.split_whitespace().collect::<Vec<_>>();
		match args.len()
		{
			2 => Ok(Noop),
			4 => Ok(Place(args[0].to_owned(),args[2].parse().unwrap(),args[3].parse().unwrap())),
			5 => Ok(Turn(args[0].to_owned(),args[2].parse().unwrap(),args[3].parse().unwrap(),args[4].parse().unwrap())),
			6 =>
			{
				let mut args = args.iter().map(|s|s.parse::<usize>()).map(Result::unwrap);
				Ok(StartingRegions
				(
					args.next().unwrap(),
					args.next().unwrap(),
					args.next().unwrap(),
					args.next().unwrap(),
					args.next().unwrap(),
					args.next().unwrap()
				))
			},
			_ => Err(ParseTurnError),
		}
	}
}

impl fmt::Display for Turn
{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result
	{
		use self::Turn::*;
		match self
		{
			&StartingRegions(a,b,c,d,e,f_) => write!(f,"{} {} {} {} {} {}",a,b,c,d,e,f_),
			&Place(ref name,region,count) => write!(f,"{} place_armies {} {}",name,region,count),
			&Turn(ref name,source,target,count) => write!(f,"{} attack/transfer {} {} {}",name,source,target,count),
			&Noop => write!(f,"No moves"),
		}
	}
}

