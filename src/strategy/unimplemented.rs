use strategy::Strategy;
use region::Region;
use turn::Turn;
use superregion::SuperRegion;

pub struct Unimplemented;

impl Strategy for Unimplemented
{
	fn set_regions<T>(&mut self,_regs: T)
		where T: IntoIterator<Item=Region>
	{
		unimplemented!();
	}

	fn region_mut<'a>(&'a mut self,_id: usize) -> Option<&'a mut Region>
	{
		unimplemented!();
	}

	fn set_superregions<T>(&mut self,_sregs: T)
		where T: IntoIterator<Item=SuperRegion>
	{
		unimplemented!();
	}

	fn turn_other<T>(&mut self,_turns: T)
		where T: IntoIterator<Item=Turn>
	{
		unimplemented!();
	}

	fn get_starting_regions<T>(&self,_avail: T) -> (usize,usize,usize,usize,usize,usize)
		where T: IntoIterator<Item=usize>
	{
		unimplemented!();
	}

	fn placement(&self,_count: usize) -> Vec<Turn>
	{
		unimplemented!();
	}

	fn turn(&self) -> Vec<Turn>
	{
		unimplemented!();
	}
}

