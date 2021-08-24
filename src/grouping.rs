use std::fs;
use chrono::prelude::*;
use crate::volume::Volume;
use crate::info::TestInfo;
use crate::info::TimeInterval;




/// There are a valid methods for aggregation
pub enum MethodGroup {
	/// by mean volume in time interval
	Mean,
	/// by last volume in time interval
	Last,
	/// by first volume in time interval
	First,
}

fn calc_mean_month (info: &Vec<TestInfo>) -> Vec<TestInfo> {
/*
let mut flag_new = true;
	let mut ret_vol: Vec<TestInfo> = Vec::new();
	ret_vol.push(info[0]);
	for i in 1..info.len() {
		for j in 0..ret_vol.len() {
			if ret_vol[j].date.year() == info[i].date.year(){
				if ret_vol[j].date.month() == info[i].date.month(){
					if ret_vol[j].date.day() < info[i].date.day(){
						std::mem::replace(&mut ret_vol[j], info[i]);
					};
					flag_new = false;
				};
			};
		}
		if flag_new {
			ret_vol.push(info[i]);
		}
		flag_new = true;
	}
	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol*/
}

fn calc_mean_year(info: &Vec<TestInfo>) -> Vec<TestInfo> {
	/*
	let mut ret_vol: Vec<TestInfo> = Vec::new();

	let (_, _, mut year) = info[0].date.get_date();
	let mut vol_sum = Volume::new(); 
	let mut count = 0.0;

	for item in info {
		let (_, _, it_year) = item.date.get_date();
		if year == it_year {
			vol_sum +=item.vol;
			count += 1.0;
		} else {
			vol_sum.mean_vol(count);
			ret_vol.push(TestInfo{
				date: DateRU::new(
					1,
					1,
					year
				),
				vol: vol_sum,
			});
			count = 0.0;
			vol_sum = Volume::new();
			year = it_year;
		}
	};

	if count != 0.0 {
		vol_sum.mean_vol(count);
		ret_vol.push(TestInfo{
				date: DateRU::new(
					1,
					1,
					year
				),
				vol: vol_sum,
			});
	}

	ret_vol*/
	todo!()
}

fn calc_last_month (info: &[TestInfo]) -> Vec<TestInfo> {

let mut flag_new = true;
	let mut ret_vol: Vec<TestInfo> = Vec::new();
	ret_vol.push(info[0]);
	for i in 1..info.len() {
		for j in 0..ret_vol.len() {
			if ret_vol[j].date.year() == info[i].date.year(){
				if ret_vol[j].date.month() == info[i].date.month(){
					if ret_vol[j].date.day() < info[i].date.day(){
						std::mem::replace(&mut ret_vol[j], info[i]);
					};
					flag_new = false;
				};
			};
		}
		if flag_new {
			ret_vol.push(info[i]);
		}
		flag_new = true;
	}
	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol
}

fn calc_last_year (info: &[TestInfo]) -> Vec<TestInfo> {

let mut flag_new = true;
	let mut ret_vol: Vec<TestInfo> = Vec::new();
	ret_vol.push(info[0]);
	for i in 1..info.len() {
		for j in 0..ret_vol.len() {
			if ret_vol[j].date.year() == info[i].date.year(){
				if ret_vol[j].date.month() < info[i].date.month(){
					std::mem::replace(&mut ret_vol[j], info[i]);
				};
				if ret_vol[j].date.month() == info[i].date.month(){
					if ret_vol[j].date.day() < info[i].date.day(){
						std::mem::replace(&mut ret_vol[j], info[i]);
					};
				};
				flag_new = false;
			};
		}
		if flag_new {
			ret_vol.push(info[i]);
		}
		flag_new = true;
	}
	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol
}

fn calc_first_month (info: &[TestInfo]) -> Vec<TestInfo> {
	
	let mut flag_new = true;
	let mut ret_vol: Vec<TestInfo> = Vec::new();
	ret_vol.push(info[0]);
	for i in 1..info.len() {
		for j in 0..ret_vol.len() {
			if ret_vol[j].date.year() == info[i].date.year(){
				if ret_vol[j].date.month() == info[i].date.month(){
					if ret_vol[j].date.day() > info[i].date.day(){
						std::mem::replace(&mut ret_vol[j], info[i]);
					};
					flag_new = false;
				};
			};
		}
		if flag_new {
			ret_vol.push(info[i]);
		}
		flag_new = true;
	}
	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol
}

fn calc_first_year (info: &[TestInfo]) -> Vec<TestInfo> {

	let mut flag_new = true;
	let mut ret_vol: Vec<TestInfo> = Vec::new();
	ret_vol.push(info[0]);
	for i in 1..info.len() {
		for j in 0..ret_vol.len() {
			if ret_vol[j].date.year() == info[i].date.year(){
				if ret_vol[j].date.month() > info[i].date.month(){
					std::mem::replace(&mut ret_vol[j], info[i]);
				};
				if ret_vol[j].date.month() == info[i].date.month(){
					if ret_vol[j].date.day() > info[i].date.day(){
						std::mem::replace(&mut ret_vol[j], info[i]);
					};
				};
				flag_new = false;
			};
		}
		if flag_new {
			ret_vol.push(info[i]);
		}
		flag_new = true;
	}
	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol
}

/// This function aggregation data from input vector for 
/// input time interval and return new vector of ```processing::info::TestInfo``` 
/// exemplars. 
pub fn grouping(
			info: &[TestInfo], 
			method: MethodGroup, 
			time: TimeInterval
			) -> Vec<TestInfo> {

	match method {
		MethodGroup::Mean => {
			match time {
				TimeInterval::Year => {
					todo!()
				},
				TimeInterval::Month => {
					todo!()
				}
			}
		},
		MethodGroup::Last => {
			match time {
				TimeInterval::Year => {
					calc_last_year(info)
				},
				TimeInterval::Month => {
					calc_last_month(info)
				}
			}
		},
		MethodGroup::First => {
			match time {
				TimeInterval::Year => {
					calc_first_year(info)
				},
				TimeInterval::Month => {
					calc_first_month(info)
				}
			}
		}
	}
}


#[test]
fn test_one () {

	let raw_data = fs::read_to_string("test_one.json").unwrap();

    let pars_data: Vec<TestInfo> = serde_json::from_str(&raw_data).unwrap();

    let month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    assert_eq!(pars_data, month_data);
}

#[test]
fn test_sort () {

	let raw_data = fs::read_to_string("test_sort.json").unwrap();

	let pars_data: Vec<TestInfo> = serde_json::from_str(&raw_data).unwrap();

    let month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    assert_eq!(pars_data, month_data);
}

#[test]
fn test_unsort () {

	let raw_data = fs::read_to_string("test_unsort.json").unwrap();
	let raw_ans = fs::read_to_string("test_unsort_ans.json").unwrap();


    let pars_data: Vec<TestInfo> = serde_json::from_str(&raw_data).unwrap();
	let pars_ans: Vec<TestInfo> = serde_json::from_str(&raw_ans).unwrap();

    let month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    assert_eq!(pars_ans, month_data);
}