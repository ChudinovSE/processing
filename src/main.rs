// use chrono::prelude::*;
use chrono::{NaiveDate, Datelike, Weekday, Duration};
// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};

use std::fs;
use processing::volume::Volume;
use processing::info::TestInfo;
use processing::info::TimeInterval;
use processing::grouping::MethodGroup;
use processing::grouping::grouping;
// use processing::interpolation::MethodInter;
// use processing::interpolation::interpolation;






fn main() {

/*
    let a = NaiveDate::from_ymd(2010, 1, 1);
    let b = NaiveDate::from_ymd(2010, 2, 1);

    println!("{}", a.month() );
    println!("{}", b.month() );
*/


    // Read and parse file with random info
    let raw_data = fs::read_to_string("test_info.json").unwrap();

    let pars_data: Vec<TestInfo> = serde_json::from_str(&raw_data).unwrap();

    println!("{}", pars_data.len());

    let month_data = grouping(&pars_data, 
            MethodGroup::Last, 
            TimeInterval::Year);
    for i in 0..month_data.len(){
        println!("{}", month_data[i]);
    }


    /*
    let month_data = grouping(&pars_data[..], 
        MethodGroup::First, 
        TimeInterval::Month);
    println!("Aggregate info for months, by first volume:");
    for i in 0..5 {
        println!("{}", month_data[i]);
    }
    println!("..........................");
    println!("");*/

}

    /*  
	// Read and parse file with random info
    let raw_data = fs::read_to_string("rnd_data.txt").unwrap();
    let mut pars_data: Vec<TestInfo> = TestInfo::pars_info(&raw_data);
    println!("Random info:");
    for i in 0..5 {
    	println!("{}", pars_data[i].print_line());
    }
    println!("..........................");
    println!("");

    // Sort random info
    pars_data.sort_by(|a, b| a.date.cmp(&b.date));
    println!("Sort by date:");
    for i in 0..5 {
    	println!("{}", pars_data[i].print_line());
    }
    println!("..........................");
    println!("");

    // Read and parse file with series info
    let raw_data = fs::read_to_string("series_data.txt").unwrap();
    let pars_data: Vec<TestInfo> = TestInfo::pars_info(&raw_data);
    println!("Series date:");
    for i in 0..5 {
    	println!("{}", pars_data[i].print_line());
    }
    println!("..........................");
    println!("");

    // Aggregate info for months, by first volume
    let month_data = grouping(&pars_data, 
    	MethodGroup::First, 
    	TimeInterval::Month);
    println!("Aggregate info for months, by first volume:");
    for i in 0..5 {
    	println!("{}", month_data[i].print_line());
    }
    println!("..........................");
    println!("");

    // Line interpolation info
    let inter_data = interpolation(&month_data, MethodInter::Line);
    println!("Line interpolation info:");
    for i in 0..5 {
    	println!("{}", inter_data[i].print_line());
    }
    println!("..........................");
    println!("");
    

}
*/