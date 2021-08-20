use std::cmp::Ordering;
use rand::Rng;

/// There are a valid intervals of time
pub enum TimeInterval{
	/// month
	Month,
	/// year
	Year,
}

/// This structure saved date in Russian format
/// # Warning! in this test project all months have 31 days! 
#[derive(Copy, Clone)]
pub struct DateRU{
	day: u8,
	month: u8,
	year: u32,
}

impl DateRU{
	/// This method generating and returning new exemplar 
	/// of DateRU with value start of unix time (01.01.1970)
	pub fn new_default() -> DateRU {
		DateRU {
			day: 1,
			month: 1,
			year: 1970,
		}
	}

	/// This method generating and returning new exemplar 
	/// of DateRU with value from input parameters: day, month and year

	/// # Panics
	/// Function panic if day > 31 or month > 12
	pub fn new(d: u8, m: u8, y: u32) -> DateRU {
		if d > 31
			|| m > 12 {
				panic!("date day {} month {} is wrong!", d, m);
			};

		DateRU{
			day: d,
			month: m,
			year: y,
		}
	}
	/// This method generating and returning new exemplar 
	/// of DateRU with random value
	pub fn new_rnd() -> DateRU {
		DateRU {
			day: rand::thread_rng().gen_range(1..32),
			month: rand::thread_rng().gen_range(1..13),
			year: rand::thread_rng().gen_range(1990..2022),
		}
	}

	/// This method change value exemplar DateRU to input 
	/// parameters: day, month and year
	pub fn set_date(&mut self, 	d: u8, m: u8, y: u32) {
		if d > 31
			|| m > 12 {
				panic!("date day {} month {} is wrong!", d, m);
			};

		self.day = d;
		self.month = m;
		self.year = y;
	}

	/// This method return tuple (u8 u8 u32) from value 
	/// (day, month and year)
	pub fn get_date(&self) -> (u8, u8, u32) {
		(self.day,
		 self.month,
		 self.year)
	}

	/// This method correct increase value of exemplar for 1 day 
	/// if it need, month and year will be increase too
	/// # Example
	/// ```
	/// use processing::date::DateRU;
	///
	/// let mut date = DateRU::new_default();
	/// assert_eq!(date.get_date(), (1, 1, 1970));
	///
	/// date.set_date(31, 8, 2021);
	/// date.add_day();
	/// assert_eq!(date.get_date(), (1, 9, 2021));
	/// ```
	pub fn add_day(&mut self) {
		self.day += 1;

		if self.day > 31 {
			self.day = 1;
			self.month += 1;
		};

		if self.month > 12 {
			self.month = 1;
			self.year += 1;
		};
	}

	/// This method return string with value for correct print
	pub fn print_date(&self) -> String {
		format!("{}.{}.{}", self.day, self.month, self.year)
	}
}

impl Eq for DateRU{}

impl PartialEq for DateRU {
	fn eq(&self, other: &Self) -> bool {
		self.day == other.day
			&& self.month == other.month
			&& self.year == other.year

	}
}

impl Ord for DateRU {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.year.cmp(&other.year) != Ordering::Equal{
			return self.year.cmp(&other.year)
		}
		else if self.month.cmp(&other.month) != Ordering::Equal{
			return self.month.cmp(&other.month)
		}
		else {
			return self.day.cmp(&other.day)
		}
	}
}

impl PartialOrd for DateRU {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}