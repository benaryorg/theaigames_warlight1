pub mod fastspread;

use request::Request;
use region::Region;
use superregion::SuperRegion;
use turn::Turn;

pub trait Strategy
{
	fn set_regions<T>(&mut self,regs: T)
		where T: IntoIterator<Item=Region>;

	fn region_mut<'a>(&'a mut self,id: usize) -> Option<&'a mut Region>;

	fn set_superregions<T>(&mut self,sregs: T)
		where T: IntoIterator<Item=SuperRegion>;

	fn turn_other<T>(&mut self,turns: T)
		where T: IntoIterator<Item=Turn>;

	fn get_starting_regions<T>(&self,avail: T) -> (usize,usize,usize,usize,usize,usize)
		where T: IntoIterator<Item=usize>;

	fn placement(&self,count: usize) -> Vec<Turn>;

	fn turn(&self) -> Vec<Turn>;

	/// Access raw events if neccessary.
	/// All events will be first sent here and if they are being consumed (return value) not
	/// further.
	/// If the event is not consumed (`false` is returned) it will be further processed by the
	/// engine.
	fn raw_event(&mut self,_event: &Request) -> (bool,Option<String>)
	{
		(false,None)
	}
}

