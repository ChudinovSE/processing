use crate::date::DateRU;
use crate::date::TimeInterval;
use crate::volume::Volume;
use crate::info::TestInfo;

pub enum MethodGroup {
	Mean,
	Last,
	First,
}

fn calc_mean_month (info: &Vec<TestInfo>) -> Vec<TestInfo> {

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

	ret_vol
}

fn calc_mean_year(info: &Vec<TestInfo>) -> Vec<TestInfo> {
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

	ret_vol
}

fn calc_last_month (info: &Vec<TestInfo>) -> Vec<TestInfo> {

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
}

fn calc_last_year (info: &Vec<TestInfo>) -> Vec<TestInfo> {

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
}

fn calc_first_month (info: &Vec<TestInfo>) -> Vec<TestInfo> {

	let mut ret_vol: Vec<TestInfo> = Vec::new();

	let (_, mut month, mut year) = info[0].date.get_date();
	let mut vol_first = info[0].vol;

	for item in info {
		let (_, it_month, it_year) = item.date.get_date();
		if month != it_month {
			ret_vol.push(TestInfo{
				date: DateRU::new(
						1,
						month,
						year
				),
				vol: vol_first,
			});
			month = it_month;
			year = it_year;
			vol_first = item.vol;
		}
	};

	ret_vol.push(TestInfo{
				date: DateRU::new(
						1,
						month,
						year
				),
				vol: vol_first,
			});

	return ret_vol
}

fn calc_first_year (info: &Vec<TestInfo>) -> Vec<TestInfo> {

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
	
	return ret_vol
}

pub fn grouping(
			info: &Vec<TestInfo>, 
			method: MethodGroup, 
			time: TimeInterval
			) -> Vec<TestInfo> {

	match method {
		MethodGroup::Mean => {
			match time {
				TimeInterval::Year => {
					calc_mean_year(info)
				},
				TimeInterval::Month => {
					calc_mean_month(info)
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