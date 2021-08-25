// use crate::date::DateRU;
use crate::volume::Volume;
use serde::{Deserialize, Serialize};

//use chrono::prelude::*;
use chrono::{NaiveDate};
use std::fmt;


/// This structure saved test info
#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct TestInfo {
	pub date: NaiveDate,
	pub vol: Volume,
}

impl TestInfo {

	/// This method generating and returning new exemplar 
    /// of TestInfo with 0 value and default date
    pub fn new() -> TestInfo{
		TestInfo{
			date: NaiveDate::from_ymd(1970, 1, 1),
			vol: Volume::new(),
		}
	}

    /// This method parse string with raw data (from file, for example)
    /// and return vector of TestInfo
    /// # Warning
    /// raw data must have format: "d.m.y. vol1 vol2 vol3 \"
	pub fn parse_info (raw_info: &String) -> Result<Vec<TestInfo>, serde_json::Error> {
       let res = serde_json::from_str(raw_info);
        match res {
            Ok (_) => res,
            Err(e) => Err(e),
        }
    }
}


impl fmt::Display for TestInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", 
            self.date,
            self.vol
            )
    }
}


/// There are a valid intervals of time
pub enum TimeInterval{
    /// month
    Month,
    /// year
    Year,
}