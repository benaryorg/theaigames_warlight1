use rawturn::RawTurn;
use region::Region;
use std::str::FromStr;
use superregion::SuperRegion;

#[derive(Debug,Clone,PartialEq)]
pub struct ParseRequestError;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum Request
{
	ListSuperRegions(Vec<SuperRegion>),
	ListRegions(Vec<Region>),
	ListNeighbours(Vec<(usize,Vec<usize>)>),
	RequestStartingRegions(Vec<usize>),
	SettingNameYou(String),
	SettingNameOther(String),
	SettingArmies(usize),
	UpdateMap(Vec<(usize,String,usize)>),
	TurnOther(Vec<RawTurn>),
	TurnPlace,
	TurnArmies,
}

impl FromStr for Request
{
	type Err = ParseRequestError;

	fn from_str(s: &str) -> Result<Self,Self::Err>
	{
		use self::Request::*;

		//FIXME: slice pattern matching
		let mut args = s.split_whitespace();
		match (args.next(),args.next())
		{
			(Some("setup_map"),Some("super_regions")) =>
			{
				let v = args.collect::<Vec<_>>();
				let v = v
					.chunks(2)
					.map(|s|s.join(" "))
					.map(|s|s.parse::<SuperRegion>())
					.map(Result::unwrap)
					.collect::<Vec<_>>();
				Ok(ListSuperRegions(v))
			},
			(Some("setup_map"),Some("regions")) =>
			{
				let v = args.collect::<Vec<_>>();
				let v = v
					.chunks(2)
					.map(|s|s.join(" "))
					.map(|s|s.parse::<Region>())
					.map(Result::unwrap)
					.collect::<Vec<_>>();
				Ok(ListRegions(v))
			},
			(Some("setup_map"),Some("neighbors")) =>
			{
				let v = args
					.collect::<Vec<_>>();
				let v = v
					.chunks(2)
					.map(|c|
						(c[0].parse::<usize>().unwrap(),c[1]
							.split(',')
							.map(|s|s.parse::<usize>())
							.map(Result::unwrap)
							.collect::<Vec<_>>()
						)
					)
					.collect::<Vec<_>>();
				Ok(ListNeighbours(v))
			},
			(Some("pick_starting_regions"),Some(_)) =>
			{
				Ok(RequestStartingRegions(
					args
						.map(|s|s.parse::<usize>())
						.map(Result::unwrap)
						.collect::<Vec<_>>()
				))
			},
			(Some("settings"),Some("your_bot")) => Ok(SettingNameYou(args.next().unwrap().to_owned())),
			(Some("settings"),Some("opponent_bot")) => Ok(SettingNameOther(args.next().unwrap().to_owned())),
			(Some("settings"),Some("starting_armies")) => Ok(SettingArmies(args.next().unwrap().parse().unwrap())),
			(Some("update_map"),Some(x)) =>
			{
				let mut v = vec![x];
				v.extend(args);
				let v = v
					.chunks(3)
					.map(|s|(s[0].parse::<usize>().unwrap(),s[1].to_owned(),s[2].parse::<usize>().unwrap()))
					.collect::<Vec<_>>();
				Ok(UpdateMap(v))
			},
			(Some("opponent_moves"),_) =>
			{
				if let Some(i) = s.find(' ')
				{
					Ok(TurnOther(s.split_at(i).1
						//TODO: not seperated by ',', must parse
						.split(',')
						.map(|s|s.parse::<RawTurn>())
						.map(|r|r.unwrap_or(RawTurn::Noop))
						.collect::<Vec<_>>()
					))
				}
				else
				{
					Ok(TurnOther(Vec::new()))
				}
			},
			(Some("go"),Some("place_armies")) => Ok(TurnPlace),
			(Some("go"),Some("attack/transfer")) => Ok(TurnArmies),
			_ => Err(ParseRequestError),
		}
	}
}

