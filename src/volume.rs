use std::ops::AddAssign;
use std::ops::Sub;

/// This structure saved volume of info
#[derive(Copy, Clone)]
pub struct Volume {
	pub vol1: f64,
	pub vol2: f64,
	pub vol3: f64,
}

impl Volume {
	/// This method generating and returning new exemplar 
	/// of Volume with 0 value 
	pub fn new() -> Volume {
		Volume{
			vol1: 0.0,
			vol2: 0.0,
			vol3: 0.0,
		}
	}

	/// This method return resuly of division volume on input parametr
	pub fn mean_vol(&mut self, numb: f64) {
		*self = Self {
			vol1: self.vol1 / numb,
			vol2: self.vol2 / numb,
			vol3: self.vol3 / numb,
		}
	}

	/// This method return string with value for correct print
	pub fn print_volume(&self) -> String {
		format!("{} {} {}", 
			self.vol1, 
			self.vol2, 
			self.vol3)
	}
}

impl AddAssign for Volume {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			vol1: self.vol1 + other.vol1,
			vol2: self.vol2 + other.vol2,
			vol3: self.vol3 + other.vol3,
		}
	}
}

impl Sub for Volume {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {
			vol1: self.vol1 - other.vol1,
			vol2: self.vol2 - other.vol2,
			vol3: self.vol3 - other.vol3,
		}
	}
}
