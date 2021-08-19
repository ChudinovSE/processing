use std::fs;

//use processing::date::DateRU;

use processing::info::TestInfo;
use processing::grouping::MethodGroup;
use processing::grouping::grouping;
use processing::interpolation::MethodInter;
use processing::interpolation::interpolation;
use processing::date::TimeInterval;
//use processing::grouping;
//use processing::interpolation;


fn main() {

    let raw_data = fs::read_to_string("series_data_test.txt").unwrap();

    let pars_data: Vec<TestInfo> = TestInfo::pars_info(&raw_data);
   // pars_data.sort_by(|a, b| a.data.cmp(&b.data));

    let year_data = grouping(&pars_data, MethodGroup::First, TimeInterval::Month);
   // println!("year_data.len {}", year_data.len());
    for i in 0..year_data.len() {
    	println!("[DEBUG] {}", year_data[i].print_line());
    }


    let inter_data = interpolation(&year_data, MethodInter::Line);

    println!("{}", inter_data.len());
    
    for i in 0 .. inter_data.len() {
    	println!("[DEBUG] {}", inter_data[i].print_line());
    }
}
