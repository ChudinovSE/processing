use std::fs;

use processing::info::TestInfo;
use processing::info::TimeInterval;
use processing::grouping::calc_mean_month;
use processing::grouping::calc_mean_month_paral;
// use processing::grouping::grouping;

use std::time::{Duration, Instant};





fn main() {

    // Read and parse file with random info
    let raw_data = fs::read_to_string("test_info.json").expect("Error read file");

    let pars_data = TestInfo::parse_info(&raw_data).expect("Error parsing json");

    println!("{}", pars_data.len());
    let start = Instant::now();
    let month_data = calc_mean_month(&pars_data);
    let duration = start.elapsed();
    println!("Without ryon {:?}", duration);
    let month_data = calc_mean_month_paral(&pars_data);
    let duration = start.elapsed();
    println!("With ryon {:?}", duration);

    /*
    for i in 0..month_data.len(){
        println!("{}", month_data[i]);
    }*/


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