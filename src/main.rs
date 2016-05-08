use std::io;
use std::io::BufReader;
use std::io::BufRead;

pub mod request;
pub mod rawturn;
pub mod turn;
pub mod region;
pub mod superregion;
pub mod player;
pub mod strategy;

use request::Request;
use rawturn::RawTurn;
use turn::Turn;
use strategy::Strategy;
use player::Player;

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

	let mut strategy = strategy::unimplemented::Unimplemented;

	let mut armies_left = 0;

	for req in input
	{
		use request::Request::*;

		let (consumed,output) = strategy.raw_event(&req);

		if let Some(output) = output
		{
			println!("{}",output);
		}

		if consumed
		{
			continue;
		}

		match req
		{
			ListSuperRegions(sregs) =>
			{
				strategy.set_superregions(sregs);
			},
			ListRegions(regs) =>
			{
				strategy.set_regions(regs);
			},
			ListNeighbours(relations) =>
			{
				for (id,neighbours) in relations
				{
					strategy.region_mut(id).unwrap().neighbours.extend(neighbours.iter());
					for n in neighbours.iter()
					{
						strategy.region_mut(*n).unwrap().neighbours.push(id);
					}
				}
			},
			RequestStartingRegions(avail) =>
			{
				let (a,b,c,d,e,f) = strategy.get_starting_regions(avail);
				println!("{}",RawTurn::StartingRegions(a,b,c,d,e,f));
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
					let mut reg = strategy.region_mut(u.0).unwrap();
					reg.player = [
						(name_you.clone(),Player::You),
						(name_other.clone(),Player::Other),
						("neutral".to_owned(),Player::Neutral)
					].iter()
						.filter(|x|x.0 == u.1)
						.map(|x|x.1)
						.next()
						.unwrap_or(Player::Unknown);
					reg.count = u.2;
				}
			},
			TurnOther(_turns) =>
			{
				//TODO: remap
			},
			TurnPlace =>
			{
				let v = strategy.placement(armies_left).iter()
					.filter_map(|t|if let &Turn::Place{region,count,..} = t
					{
						strategy.region_mut(region).unwrap().count -= count;
						Some(RawTurn::Place
						{
							name: name_you.clone(),
							region: region,
							count: count,
						})
					}
					else
					{
						None
					})
					.map(|t|format!("{}",t))
					.collect::<Vec<_>>();
				if v.len() <= 0
				{
					println!("{}",RawTurn::Noop);
				}
				else
				{
					println!("{}",v.join(","));
				}
			},
			TurnArmies =>
			{
				let v = strategy.turn().iter()
					.filter_map(|t|if let &Turn::Turn{source,target,count,..} = t
					{
						Some(RawTurn::Turn
						{
							name: name_you.clone(),
							count: count,
							source: source,
							target: target,
						})
					}
					else
					{
						None
					})
					.map(|t|format!("{}",t))
					.collect::<Vec<_>>();
				if v.len() <= 0
				{
					println!("{}",RawTurn::Noop);
				}
				else
				{
					println!("{}",v.join(","));
				}
			},
		}
	}
}

