use std::io;
use std::io::BufReader;
use std::io::BufRead;
use request::Request;
use turn::Turn;

mod request;
mod turn;

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

	for req in input
	{
		use request::Request::*;
		match req
		{
			ListSuperRegions(sregions) =>
			{
			},
			ListRegions(regions) =>
			{
			},
			ListNeighbours(relations) =>
			{
			},
			RequestStartingRegions(available) =>
			{
			},
			SettingNameYou(name) =>
			{
			},
			SettingNameOther(name) =>
			{
			},
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

