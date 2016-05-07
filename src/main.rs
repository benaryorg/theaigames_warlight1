use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

mod request;
mod turn;
mod region;
mod superregion;

use request::Request;
use turn::Turn;
use region::Region;
use superregion::SuperRegion;

fn main()
{
	let input = io::stdin();
	let input = BufReader::new(input);
	let input = input.lines();
	let input = input
		.map(Result::unwrap)
		.filter(|s|s.len()>0)
		.map(|s|s.parse::<Request>())
		.map(Result::unwrap);

	let mut name_you = String::new();
	let mut name_other = String::new();

	let mut sup_regions: HashMap<usize,SuperRegion> = HashMap::new();
	let mut regions: HashMap<usize,Region> = HashMap::new();

	for req in input
	{
		use request::Request::*;
		match req
		{
			ListSuperRegions(sregions) =>
			{
				sup_regions.extend(sregions.iter().map(|sr|(sr.id,sr.clone())));
			},
			ListRegions(regs) =>
			{
				regions.extend(regs.iter().map(|r|(r.id,r.clone())));
			},
			ListNeighbours(relations) =>
			{
				for (id,neighbours) in relations
				{
					regions.get_mut(&id).unwrap().neighbours.extend(neighbours.iter());
					for n in neighbours
					{
						regions.get_mut(&n).unwrap().neighbours.push(id);
					}
				}
			},
			RequestStartingRegions(_avail) =>
			{
				println!("give me randomly");
			},
			SettingNameYou(name) => name_you = name,
			SettingNameOther(name) => name_other = name,
			SettingArmies(count) =>
			{
			},
			UpdateMap(updates) =>
			{
			},
			TurnOther(turns) =>
			{
			},
			TurnPlace =>
			{
			},
			TurnArmies =>
			{
			},
		}
	}
}

