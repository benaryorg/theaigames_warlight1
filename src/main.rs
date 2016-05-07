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
				let mut v = Vec::new();
				{
					let regs = regions.values()
						.filter(|r|r.player == name_you)
						.filter(|r|r.neighbours.iter()
							.map(|o|regions.get(o))
							.map(Option::unwrap)
							.filter(|o|o.player == name_other)
							.any(|o|o.count >= r.count)
						)
						.take(3)
						.collect::<Vec<_>>();

					let count = if regs.len() > 0
					{
						armies_left/regs.len()
					}
					else
					{
						1
					};
					for r in regs.iter()
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
					while armies_left > 0
					{
						for r in regions.values()
							.filter(|r|r.player == name_you)
						{
							if armies_left <= 0
							{
								break;
							}
							for n in regions.get(&r.id).unwrap().neighbours.iter()
							{
								if regions.get(&n).unwrap().player != name_you
								{
									v.push(
										Turn::Place
										{
											name: name_you.clone(),
											region: r.id,
											count: 1,
										}
									);
									armies_left -= 1;
									break;
								}
							}
						}
					}
				}
				if v.len() <= 0
				{
					v.push(Turn::Noop);
				}
				{
					let v = v.iter()
						.filter(|t|
							if let &&Turn::Place{count,..} = t
							{
								count > 0
							}
							else
							{
								true
							}
						)
						.map(|t|format!("{}",t))
						.collect::<Vec<_>>();
					println!("{}",v.join(","));
				}
				for r in v.iter()
				{
					if let &Turn::Place{region,count,..} = r
					{
						regions.get_mut(&region).unwrap().count += count;
					}
				}
			},
			TurnArmies =>
			{
				let mut v = Vec::new();
				let regs = regions.values()
					.filter(|r|r.player == name_you)
					.filter_map(|r|
					{
						let x =r.neighbours.iter()
						.map(|o|regions.get(o))
						.map(Option::unwrap)
						.filter(|o|o.player != name_you)
						.filter(|o|o.count*5 <= r.count*2)
						.next()
						.map(|o|o.id);
						if let Some(x) = x
						{
							Some(((r.id,(r.count*2)/3),x))
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
				if v.len() <= 0
				{
					v.push(Turn::Noop);
				}
				let v = v.iter().map(|t|format!("{}",t)).collect::<Vec<_>>();
				println!("{}",v.join(","));
			},
		}
	}
}

