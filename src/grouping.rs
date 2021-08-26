use chrono::prelude::*;
use std::collections::HashMap;
use rayon::prelude::*;

use crate::info::TestInfo;

/// There are a valid intervals of time
pub enum TimeInterval{
    /// month
    Month,
    /// year
    Year,
}

/// There are a valid methods for aggregation
pub enum MethodGroup {
	/// by mean volume in time interval
	Mean,
	/// by last volume in time interval
	Last,
	/// by first volume in time interval
	First,
}

fn calc_mean_month_paral (info: &[TestInfo]) -> Vec<TestInfo> {

	type ResMap = HashMap::<(i32, u32), (TestInfo, u32)>;


	let mut parts = Vec::new();
	parts.push(&info[0..info.len()/4]);
	parts.push(&info[info.len()/4..info.len()/2]);
	parts.push(&info[info.len()/2..info.len()/2 + info.len()/4]);
	parts.push(&info[info.len()/2 + info.len()/4..info.len()]);

	let tmp: Vec<ResMap> = parts.par_iter().map(|part|{
		let mut ltmp = ResMap::new();
		let _: Vec<()> = part.iter().map(|it| {
		let ym = (it.date.year(), it.date.month());
		match ltmp.get_mut(&ym) {
			Some(x) => {
						x.0.vol += it.vol;
						x.1 += 1;
					},
			None => {
					ltmp.insert(ym,(*it, 1));
				}
			};
		}).collect();
		ltmp}).collect();

	let mut tmp_rez = ResMap::new();

	let _: Vec<()> = tmp.iter().map(|it|{
		for (key, val) in it.iter() {
			match tmp_rez.get_mut(&*key) {
			Some(x) => {
						x.0.vol += val.0.vol;
						x.1 += 1;
					},
			None => {
					tmp_rez.insert(*key,(val.0, val.1));
				}
			};
		};
	}).collect();

	let mut ret_vol: Vec<TestInfo> = tmp_rez.into_par_iter()
				.map(|(_date, mut info)| {
					info.0.vol = info.0.vol.mean_vol(info.1 as f64);
					info.0
				})
				.collect();

	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol
}

fn calc_mean_month (info: &[TestInfo]) -> Vec<TestInfo> {

	let mut tmp = HashMap::<(i32, u32), (TestInfo, u32)>::new();

	let _: Vec<()> = info.iter().map(|it| {
		let ym = (it.date.year(), it.date.month());
		match tmp.get_mut(&ym) {
			Some(x) => {
						x.0.vol += it.vol;
						x.1 += 1;
					},
			None => {
					tmp.insert(ym,(*it, 1));
			}
		};
	}
	).collect();

	let mut ret_vol: Vec<TestInfo> = tmp.into_iter()
				.map(|(_date, mut info)| {
					info.0.vol = info.0.vol.mean_vol(info.1 as f64);
					info.0
				})
				.collect();

	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol
}

pub fn calc_mean_year_paral (info: &[TestInfo]) -> Vec<TestInfo> {

	type ResMap = HashMap::<i32, (TestInfo, u32)>;


	let mut parts = Vec::new();
	parts.push(&info[0..info.len()/4]);
	parts.push(&info[info.len()/4..info.len()/2]);
	parts.push(&info[info.len()/2..info.len()/2 + info.len()/4]);
	parts.push(&info[info.len()/2 + info.len()/4..info.len()]);

	let tmp: Vec<ResMap> = parts.par_iter().map(|part|{
		let mut ltmp = ResMap::new();
		let _: Vec<()> = part.iter().map(|it| {
		match ltmp.get_mut(&it.date.year()) {
			Some(x) => {
						x.0.vol += it.vol;
						x.1 += 1;
					},
			None => {
					ltmp.insert(it.date.year(),(*it, 1));
				}
			};
		}).collect();
		ltmp}).collect();

	let mut tmp_rez = ResMap::new();

	let _: Vec<()> = tmp.iter().map(|it|{
		for (key, val) in it.iter() {
			match tmp_rez.get_mut(&*key) {
			Some(x) => {
						x.0.vol += val.0.vol;
						x.1 += 1;
					},
			None => {
					tmp_rez.insert(*key,(val.0, val.1));
				}
			};
		};
	}).collect();

	let mut ret_vol: Vec<TestInfo> = tmp_rez.into_par_iter()
				.map(|(_date, mut info)| {
					info.0.vol = info.0.vol.mean_vol(info.1 as f64);
					info.0
				})
				.collect();

	ret_vol.sort_by(|a, b| a.date.cmp(&b.date));
	ret_vol
}

fn calc_mean_year(info: &[TestInfo]) -> Vec<TestInfo> {

	let mut tmp = HashMap::<i32, (TestInfo, u32)>::new();

	let _: Vec<()> = info.iter().map(|it| {
		match tmp.get_mut(&it.date.year()) {
			Some(x) => {
						x.0.vol += it.vol;
						x.1 += 1;
					},
			None => {
					tmp.insert(it.date.year(),(*it, 1));
			}
		};
	}
	).collect();

	let ret_vol: Vec<TestInfo> = tmp.into_iter()
				.map(|(_date, mut info)| {
					info.0.vol = info.0.vol.mean_vol(info.1 as f64);
					info.0
				})
				.collect();
	ret_vol
}

fn calc_last_month_paral (info: &[TestInfo]) -> Vec<TestInfo> {

	type ResMap = HashMap::<(i32, u32), TestInfo>;

	let mut parts = Vec::new();
	parts.push(&info[0..info.len()/4]);
	parts.push(&info[info.len()/4..info.len()/2]);
	parts.push(&info[info.len()/2..info.len()/2 + info.len()/4]);
	parts.push(&info[info.len()/2 + info.len()/4..info.len()]);

	let tmp: Vec<ResMap> = parts.par_iter().map(|part|{
		let mut ltmp = ResMap::new();
		let _: Vec<()> = part.iter().map(|it| {
		let ym = (it.date.year(), it.date.month());
		match ltmp.get_mut(&ym) {
			Some(x) => {
						if x.date.day() < it.date.day() {
							*x = *it;
						}
						},
			None => {
					ltmp.insert(ym,*it);
					}
			};
		}).collect();
		ltmp}).collect();

	let mut tmp_rez = ResMap::new();

	let _: Vec<()> = tmp.iter().map(|it|{
		for (key, val) in it.iter() {
			match tmp_rez.get_mut(&*key) {
			Some(x) => {
						if x.date.day() < val.date.day() {
							*x = *val;
						}
					},
			None => {
					tmp_rez.insert(*key, *val);
				}
			};
		};
	}).collect();

	let ret_vol: Vec<TestInfo> = tmp_rez.into_iter()
				.map(|(_date, info)|info)
				.collect();
	ret_vol
}

fn calc_last_month (info: &[TestInfo]) -> Vec<TestInfo> {

	let mut tmp = HashMap::<(i32, u32), TestInfo>::new();

	let _: Vec<()> = info.iter().map(|it| {
		let ym = (it.date.year(), it.date.month());
		match tmp.get_mut(&ym) {
			Some(x) => {
						if x.date.day() < it.date.day() {
							*x = *it;
						}
					},
			None => {
					tmp.insert(ym,*it);
			}
		};
	}
	).collect();

	let ret_vol: Vec<TestInfo> = tmp.into_iter()
				.map(|(_date, info)| info)
				.collect();
	ret_vol
}

fn calc_last_year_paral (info: &[TestInfo]) -> Vec<TestInfo> {

	type ResMap = HashMap::<i32, TestInfo>;

	let mut parts = Vec::new();
	parts.push(&info[0..info.len()/4]);
	parts.push(&info[info.len()/4..info.len()/2]);
	parts.push(&info[info.len()/2..info.len()/2 + info.len()/4]);
	parts.push(&info[info.len()/2 + info.len()/4..info.len()]);

	let tmp: Vec<ResMap> = parts.par_iter().map(|part|{
		let mut ltmp = ResMap::new();
		let _: Vec<()> = part.iter().map(|it| {
		match ltmp.get_mut(&it.date.year()) {
			Some(x) => {
						if x.date.month() == it.date.month() {
							if x.date.day() < it.date.day() {
								*x = *it;
							}
						}
						if x.date.month() < it.date.month() {
							*x = *it;
						}
						},
			None => {
					ltmp.insert(it.date.year(),*it);
					}
			};
		}).collect();
		ltmp}).collect();

	let mut tmp_rez = ResMap::new();

	let _: Vec<()> = tmp.iter().map(|it|{
		for (key, val) in it.iter() {
			match tmp_rez.get_mut(&*key) {
			Some(x) => {
						if x.date.month() == val.date.month() {
							if x.date.day() < val.date.day() {
								*x = *val;
							}
						}
						if x.date.month() < val.date.month() {
							*x = *val;
						}
					},
			None => {
					tmp_rez.insert(*key, *val);
				}
			};
		};
	}).collect();

	let ret_vol: Vec<TestInfo> = tmp_rez.into_iter()
				.map(|(_date, info)|info)
				.collect();
	ret_vol
}

fn calc_last_year (info: &[TestInfo]) -> Vec<TestInfo> {

	let mut tmp = HashMap::<i32, TestInfo>::new();

	let _: Vec<()> = info.iter().map(|it| {
		match tmp.get_mut(&it.date.year()) {
			Some(x) => {
						if x.date.month() == it.date.month() {
							if x.date.day() < it.date.day() {
								*x = *it;
							}
						}
						if x.date.month() < it.date.month() {
							*x = *it;
						}
					},
			None => {
					tmp.insert(it.date.year(),*it);
			}
		};
	}
	).collect();

	let ret_vol: Vec<TestInfo> = tmp.into_iter()
				.map(|(_date, info)| info)
				.collect();
	ret_vol
}

fn calc_first_month_paral (info: &[TestInfo]) -> Vec<TestInfo> {

	type ResMap = HashMap::<(i32, u32), TestInfo>;

	let mut parts = Vec::new();
	parts.push(&info[0..info.len()/4]);
	parts.push(&info[info.len()/4..info.len()/2]);
	parts.push(&info[info.len()/2..info.len()/2 + info.len()/4]);
	parts.push(&info[info.len()/2 + info.len()/4..info.len()]);

	let tmp: Vec<ResMap> = parts.par_iter().map(|part|{
		let mut ltmp = ResMap::new();
		let _: Vec<()> = part.iter().map(|it| {
		let ym = (it.date.year(), it.date.month());
		match ltmp.get_mut(&ym) {
			Some(x) => {
						if x.date.day() > it.date.day() {
							*x = *it;
						}
						},
			None => {
					ltmp.insert(ym,*it);
					}
			};
		}).collect();
		ltmp}).collect();

	let mut tmp_rez = ResMap::new();

	let _: Vec<()> = tmp.iter().map(|it|{
		for (key, val) in it.iter() {
			match tmp_rez.get_mut(&*key) {
			Some(x) => {
						if x.date.day() > val.date.day() {
							*x = *val;
						}
					},
			None => {
					tmp_rez.insert(*key, *val);
				}
			};
		};
	}).collect();

	let ret_vol: Vec<TestInfo> = tmp_rez.into_iter()
				.map(|(_date, info)|info)
				.collect();
	ret_vol
}

fn calc_first_month (info: &[TestInfo]) -> Vec<TestInfo> {

	let mut tmp = HashMap::<(i32, u32), TestInfo>::new();

	let _: Vec<()> = info.iter().map(|it| {
		let ym = (it.date.year(), it.date.month());
		match tmp.get_mut(&ym) {
			Some(x) => {
						if x.date.day() > it.date.day() {
							*x = *it;
						}
					},
			None => {
					tmp.insert(ym,*it);
			}
		};
	}
	).collect();

	let ret_vol: Vec<TestInfo> = tmp.into_iter()
				.map(|(_date, info)| info)
				.collect();
	ret_vol

}

fn calc_first_year_paral (info: &[TestInfo]) -> Vec<TestInfo> {

	type ResMap = HashMap::<i32, TestInfo>;

	let mut parts = Vec::new();
	parts.push(&info[0..info.len()/4]);
	parts.push(&info[info.len()/4..info.len()/2]);
	parts.push(&info[info.len()/2..info.len()/2 + info.len()/4]);
	parts.push(&info[info.len()/2 + info.len()/4..info.len()]);

	let tmp: Vec<ResMap> = parts.par_iter().map(|part|{
		let mut ltmp = ResMap::new();
		let _: Vec<()> = part.iter().map(|it| {
		match ltmp.get_mut(&it.date.year()) {
			Some(x) => {
						if x.date.month() == it.date.month() {
							if x.date.day() > it.date.day() {
								*x = *it;
							}
						}
						if x.date.month() > it.date.month() {
							*x = *it;
						}
						},
			None => {
					ltmp.insert(it.date.year(),*it);
					}
			};
		}).collect();
		ltmp}).collect();

	let mut tmp_rez = ResMap::new();

	let _: Vec<()> = tmp.iter().map(|it|{
		for (key, val) in it.iter() {
			match tmp_rez.get_mut(&*key) {
			Some(x) => {
						if x.date.month() == val.date.month() {
							if x.date.day() > val.date.day() {
								*x = *val;
							}
						}
						if x.date.month() > val.date.month() {
							*x = *val;
						}
					},
			None => {
					tmp_rez.insert(*key, *val);
				}
			};
		};
	}).collect();

	let ret_vol: Vec<TestInfo> = tmp_rez.into_iter()
				.map(|(_date, info)|info)
				.collect();
	ret_vol
}

fn calc_first_year (info: &[TestInfo]) -> Vec<TestInfo> {

	let mut tmp = HashMap::<i32, TestInfo>::new();

	let _: Vec<()> = info.iter().map(|it| {
		match tmp.get_mut(&it.date.year()) {
			Some(x) => {
						if x.date.month() == it.date.month() {
							if x.date.day() > it.date.day() {
								*x = *it;
							}
						}
						if x.date.month() > it.date.month() {
							*x = *it;
						}
					},
			None => {
					tmp.insert(it.date.year(),*it);
			}
		};
	}
	).collect();

	let mut ret_vol: Vec<TestInfo> = tmp.into_iter()
				.map(|(_date, info)| info)
				.collect();

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
					if info.len() < 4096 {
						calc_mean_year(info)
					} else {
						calc_mean_year_paral(info)
					}
				},
				TimeInterval::Month => {
					if info.len() < 4096 {
						calc_mean_month(info)
					} else {
						calc_mean_month_paral(info)
					}
				}
			}
		},
		MethodGroup::Last => {
			match time {
				TimeInterval::Year => {
					if info.len() < 4096 {
						calc_last_year(info)
					} else {
						calc_last_year_paral(info)
					}
				},
				TimeInterval::Month => {
					if info.len() < 4096 {
						calc_last_month(info)
					} else {
						calc_last_month_paral(info)
					}
				}
			}
		},
		MethodGroup::First => {
			match time {
				TimeInterval::Year => {
					if info.len() < 4096 {
						calc_first_year(info)
					} else {
						calc_first_year_paral(info)
					}
				},
				TimeInterval::Month => {
					if info.len() < 4096 {
						calc_first_month(info)
					} else {
						calc_first_month_paral(info)
					}
				}
			}
		}
	}
}


#[test]
fn test_one () {
	use std::fs;
	let raw_data = fs::read_to_string("test_one.json").unwrap();

    let pars_data: Vec<TestInfo> = serde_json::from_str(&raw_data).unwrap();

    let month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    assert_eq!(pars_data, month_data);
}

#[test]
fn test_sort () {
	use std::fs;
	let raw_data = fs::read_to_string("test_sort.json").unwrap();

	let pars_data: Vec<TestInfo> = serde_json::from_str(&raw_data).unwrap();

    let mut month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    month_data.sort_by(|a, b| a.date.cmp(&b.date));

    assert_eq!(pars_data, month_data);
}

#[test]
fn test_unsort () {
	use std::fs;
	let raw_data = fs::read_to_string("test_unsort.json").unwrap();
	let raw_ans = fs::read_to_string("test_unsort_ans.json").unwrap();


    let pars_data: Vec<TestInfo> = serde_json::from_str(&raw_data).unwrap();
	let pars_ans: Vec<TestInfo> = serde_json::from_str(&raw_ans).unwrap();

    let mut month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    month_data.sort_by(|a, b| a.date.cmp(&b.date));

    assert_eq!(pars_ans, month_data);
}