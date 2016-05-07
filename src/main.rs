use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use request::Request;
use turn::Turn;

mod request;
mod turn;
mod region;
mod superregion;

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

	let mut sup_regions = HashMap::new();
	let mut regions = HashMap::new();

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

