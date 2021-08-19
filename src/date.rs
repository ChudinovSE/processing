use std::cmp::Ordering;
use rand::Rng;

pub enum TimeInterval{
	Month,
	Year,
}

#[derive(Copy, Clone)]
pub struct DateRU{
	day: u8,
	month: u8,
	year: u32,
}

impl DateRU{
	pub fn new_default() -> DateRU {
		DateRU {
			day: 1,
			month: 1,
			year: 1970,
		}
	}

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

	pub fn new_rnd() -> DateRU {
		DateRU {
			day: rand::thread_rng().gen_range(1..32),
			month: rand::thread_rng().gen_range(1..13),
			year: rand::thread_rng().gen_range(1990..2022),
		}
	}

	pub fn set_date(&mut self, 	d: u8, m: u8, y: u32) {
		if d > 31
			|| m > 12 {
				panic!("date day {} month {} is wrong!", d, m);
			};

		self.day = d;
		self.month = m;
		self.year = y;
	}

	pub fn get_date(&self) -> (u8, u8, u32) {
		(self.day,
		 self.month,
		 self.year)
	}

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