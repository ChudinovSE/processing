use crate::date::DateRU;
use crate::volume::Volume;

/// This structure saved test info
pub struct TestInfo {
    /// date in Russian format
	pub date: DateRU,
    /// volume of info
	pub vol: Volume,
}

impl TestInfo {

	/// This method generating and returning new exemplar 
    /// of TestInfo with 0 value and default date
    pub fn new() -> TestInfo{
		TestInfo{
			date: DateRU::new_default(),
			vol: Volume::new(),
		}
	}

    /// This method return string with value and date of TestInfo in 
    /// line for correct print    
	pub fn print_line(&self) -> String {
		format!("{} {}", 
			self.date.print_date(), 
			self.vol.print_volume())
	}

    /// This method parse string with raw data (from file, for example)
    /// and return vector of TestInfo
    /// # Warning
    /// raw data must have format:
    /// ```
    ///   Date   vol1 vol2 vol3
    /// 1.1.2000 3885 25581 1222 
    /// 2.1.2000 3847 37062 6616
    /// ....................
    /// ```
	pub fn pars_info (raw_info: &String) -> Vec<TestInfo> {
    
    	let mut pars_info: Vec<TestInfo> = Vec::new();
   		for line in  raw_info.lines().collect::<Vec<&str>>() {
    		let a: Vec<&str> = line
    			.split(' ')
        		.filter(|s| !s.is_empty())        
        		.collect();

        	let b: Vec<i32> = a[0]
     			.split('.')
     			.map(|s| s.parse().unwrap())
     			.collect();

     		let c: TestInfo = TestInfo {
     				date: DateRU::new(
                        b[0] as u8,
     					b[1] as u8,
     					b[2] as u32
     				),
     				vol: Volume {
     					vol1: a[1].parse::<f64>().unwrap(),
     					vol2: a[2].parse::<f64>().unwrap(),
     					vol3: a[3].parse::<f64>().unwrap(),
     				}
     		};
     		pars_info.push(c);	
     	};
     	pars_info
	}
}