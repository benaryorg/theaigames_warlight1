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
	std::env::set_var("RUST_BACKTRACE","1");
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

	let mut armies_left = 0;

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
				armies_left = count;
			},
			UpdateMap(updates) =>
			{
				for u in updates
				{
					let mut reg = regions.get_mut(&u.0).unwrap();
					reg.player = u.1;
					reg.count = u.2;
				}
			},
			TurnOther(_turns) =>
			{
				//TODO: what the hell?
			},
			TurnPlace =>
			{
				let regs = regions.values()
					.filter(|r|r.player.eq(&name_you))
					.filter(|r|r.neighbours.iter()
						.map(|o|regions.get(o))
						.map(Option::unwrap)
						.filter(|o|o.player != name_you)
						.any(|o|o.count*2 < r.count)
					).collect::<Vec<_>>();

				let mut v = Vec::new();
				if armies_left > regs.len()
				{
					let count = if regs.len() > 0
					{
						armies_left/regs.len()
					}
					else
					{
						1
					};
					for r in regs
					{
						v.push(
							Turn::Place
							{
								name: name_you.clone(),
								region: r.id,
								count: count,
							}
						);
						armies_left -= count;
					}
					v.push(
						Turn::Place
						{
							name: name_you.clone(),
							region: regions.values().max_by_key(|r|r.count).unwrap().id,
							count: armies_left,
						}
					);
				}
				else
				{
					for r in regs
					{
						if armies_left <= 0
						{
							break;
						}
						v.push(
							Turn::Place
							{
								name: name_you.clone(),
								region: r.id,
								count: 1,
							}
						);
						armies_left -= 1;
					}
				}
				let v = v.iter().map(|t|format!("{}",t)).collect::<Vec<_>>();
				println!("{}",v.join(","));
			},
			TurnArmies =>
			{
				let mut v = Vec::new();
				let regs = regions.values()
					.filter(|r|r.player.eq(&name_you))
					.filter_map(|r|
					{
						let x =r.neighbours.iter()
						.map(|o|regions.get(o))
						.map(Option::unwrap)
						.filter(|o|o.player != name_you)
						.filter(|o|o.count*3 < r.count)
						.next()
						.map(|o|o.id);
						if let Some(x) = x
						{
							Some(((r.id,r.count*2/3),x))
						}
						else
						{
							None
						}
					})
					.collect::<Vec<_>>();
				for ((r,count),n) in regs
				{
					v.push(
						Turn::Turn
						{
							name: name_you.clone(),
							source: r,
							target: n,
							count: count,
						}
					);
				}
			},
		}
	}
}

