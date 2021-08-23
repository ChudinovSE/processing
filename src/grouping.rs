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
	let mut ret_vol: Vec<TestInfo> = Vec::new();
	let  (_, mut month, mut year) = info[0].date.get_date();
	let mut vol_sum = Volume::new(); 
	let mut count = 0.0;

	for item in info {
		let (_, it_month, it_year) = item.date.get_date();
		if month == it_month {
			vol_sum +=item.vol;
			count += 1.0;
		} else {
			vol_sum.mean_vol(count);
			ret_vol.push(TestInfo{
				date: DateRU::new(
					1,
					month,
					year
				),
				vol: vol_sum,
			});
			count = 0.0;
			vol_sum = Volume::new();
			month = it_month;
			year = it_year;
		}
	};

	if count != 0.0 {
		vol_sum.mean_vol(count);
		ret_vol.push(TestInfo{
				date: DateRU::new(
					1,
					month,
					year
				),
				vol: vol_sum,
			});
	}

	ret_vol*/
	todo!()
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

fn calc_last_month (info: &Vec<TestInfo>) -> Vec<TestInfo> {
/*
	let mut ret_vol: Vec<TestInfo> = Vec::new();

	let (_, mut month, mut year) = info[0].date.get_date();
	let mut vol_last = Volume::new();
		for item in info {
			let (_, it_month, it_year) = item.date.get_date();
			if month == it_month {
				vol_last = item.vol;
			} else {
				ret_vol.push(TestInfo{
					date: DateRU::new(
						1,
						month,
						year
					),
					vol: vol_last,
				});
				month = it_month;
				year = it_year;
				vol_last = item.vol;
			}
		}
		if (1, month, year) != ret_vol[ret_vol.len() - 1].date.get_date(){
				ret_vol.push(TestInfo{
					date: DateRU::new(
						1,
						month,
						year
					),
					vol: vol_last,
				});
			}

	return ret_vol
	*/
	todo!()
}

fn calc_last_year (info: &Vec<TestInfo>) -> Vec<TestInfo> {
	/*

	let mut ret_vol: Vec<TestInfo> = Vec::new();

	let (_, _, mut year) = info[0].date.get_date();
	let mut vol_last = Volume::new();
		for item in info {
			let (_, _, it_year) = item.date.get_date();
			if year == it_year {
				vol_last = item.vol;
			} else {
				ret_vol.push(TestInfo{
					date: DateRU::new(
						1,
						1,
						year
					),
					vol: vol_last,
				});
				year = it_year;
				vol_last = item.vol;
			}
		}

		if (1, 1, year) != ret_vol[ret_vol.len() - 1].date.get_date(){
				ret_vol.push(TestInfo{
					date: DateRU::new(
						1,
						1,
						year
					),
					vol: vol_last,
				});
			}

	return ret_vol
	*/
	todo!()
}

fn calc_first_month (info: &[TestInfo]) -> Vec<TestInfo> {

	todo!()
/*
	let mut ret_vol = Vec::new();
	ret_vol.push(info[0]);
	for i in 1..=info.len() {
		for item in ret_vol {
			if item
		}
	}*/
}

fn calc_first_year (info: &Vec<TestInfo>) -> Vec<TestInfo> {
/*
	let mut ret_vol: Vec<TestInfo> = Vec::new();

	let (_, _, mut year) = info[0].date.get_date();
	let mut vol_first = info[0].vol;

	for item in info {
		let (_, _, it_year) = item.date.get_date();
		if year != it_year {
			ret_vol.push(TestInfo{
				date: DateRU::new(
						1,
						1,
						year
				),
				vol: vol_first,
			});
			year = it_year;
			vol_first = item.vol;
		}
	};

	ret_vol.push(TestInfo{
				date: DateRU::new(
						1,
						1,
						year
				),
				vol: vol_first,
			});
	
	return ret_vol*/
	todo!()
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
					todo!()
				},
				TimeInterval::Month => {
					todo!()
				}
			}
		},
		MethodGroup::First => {
			match time {
				TimeInterval::Year => {
					todo!()
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

    let pars_data = TestInfo::pars_info(&raw_data);

    let month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    assert_eq!(pars_data, month_data);
}

#[test]
fn test_sort () {

	let raw_data = fs::read_to_string("test_sort.json").unwrap();
	let raw_ans = fs::read_to_string("test_sort_ans.json").unwrap();


    let pars_data = TestInfo::pars_info(&raw_data);
    let pars_ans = TestInfo::pars_info(&raw_ans);

    let month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    assert_eq!(pars_ans, month_data);
}

#[test]
fn test_unsort () {

	let raw_data = fs::read_to_string("test_unsort.json").unwrap();
	let raw_ans = fs::read_to_string("test_unsort_ans.json").unwrap();


    let pars_data = TestInfo::pars_info(&raw_data);
    let pars_ans = TestInfo::pars_info(&raw_ans);

    let month_data = grouping(&pars_data, 
        MethodGroup::First, 
        TimeInterval::Month);

    assert_eq!(pars_ans, month_data);
}