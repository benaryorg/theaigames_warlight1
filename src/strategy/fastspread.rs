use std::collections::BTreeMap;
use std::iter::FromIterator;

use region::Region as Location;
use request::Request;
use strategy::Strategy;
use superregion::SuperRegion;
use turn::Turn;

#[derive(Debug,Clone,PartialEq)]
struct Region
{
	loc: Location,
	distances: BTreeMap<usize,(usize,usize)>,
}

impl Region
{
	fn new(loc: Location) -> Region
	{
		Region
		{
			loc: loc,
			distances: BTreeMap::new(),
		}
	}

	/// BTreeMap<target,(len,via)>
	fn update(&mut self,other: usize,table: &BTreeMap<usize,(usize,usize)>) -> bool
	{
		let table2 = BTreeMap::from_iter(table.clone().into_iter().chain(self.loc.neighbours.iter().map(|&n|(n,(1,n)))));
		let mut updated = table2.ne(&table);
		let table = table2;

		for (&id,&(distance,_)) in table.iter()
		{
			let distance = distance+1;
			if let Some(&(current,_)) = self.distances.get(&id)
			{
				if distance < current
				{
					self.distances.insert(id,(distance,other));
					updated = true;
				}
			}
			else
			{
				self.distances.insert(id,(distance,other));
				updated = true;
			}
		}

		updated
	}
}

pub struct FastSpread
{
	regions: BTreeMap<usize,Region>,
	super_regions: BTreeMap<usize,SuperRegion>,
}

impl FastSpread
{
	pub fn new() -> Self
	{
		FastSpread
		{
			regions: BTreeMap::new(),
			super_regions: BTreeMap::new(),
		}
	}

	fn init_distances(&mut self)
	{
		let mut updated = true;
		while updated
		{
			updated = false;
			let regions_immutable = self.regions.clone();
			for reg in regions_immutable.values()
			{
				for &other in reg.loc.neighbours.iter()
				{
					let table = self.regions[&other].distances.clone();
					if self.regions.get_mut(&reg.loc.id).unwrap().update(other,&table)
					{
						updated = true;
					}
				}
			}
		}
	}
}

impl Strategy for FastSpread
{
	fn set_regions<T>(&mut self,regs: T)
		where T: IntoIterator<Item=Location>
	{
		self.regions.extend
		(
			regs.into_iter()
				.map(|l|(l.id,Region::new(l)))
		);
	}

	fn region_mut<'a>(&'a mut self,id: usize) -> Option<&'a mut Location>
	{
		self.regions.get_mut(&id).map(|r|&mut r.loc)
	}

	fn set_superregions<T>(&mut self,sregs: T)
		where T: IntoIterator<Item=SuperRegion>
	{
		self.super_regions.extend
		(
			sregs.into_iter()
				.map(|sr|(sr.id,sr))
		);
	}

	fn turn_other<T>(&mut self,_turns: T)
		where T: IntoIterator<Item=Turn>
	{
		//TODO: to something maybe?
	}

	fn get_starting_regions<T>(&self,_avail: T) -> Option<(usize,usize,usize,usize,usize,usize)>
		where T: IntoIterator<Item=usize>
	{
		//TODO: choose starting regions
		None
	}

	fn placement(&self,_count: usize) -> Vec<Turn>
	{
		unimplemented!();
	}

	fn turn(&self) -> Vec<Turn>
	{
		unimplemented!();
	}

	fn raw_event_after(&mut self,req: &Request) -> Option<String>
	{
		if let &Request::ListNeighbours(_) = req
		{
			self.init_distances();
		}
		None
	}
}

