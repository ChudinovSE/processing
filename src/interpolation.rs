use crate::volume::Volume;
use crate::info::TestInfo;


pub enum MethodInter {
	Line,
	Poly3,
}

fn calc_step (first: &TestInfo, second: &TestInfo) -> (Volume, u32) {
	let (_, first_month, first_year) = first.date.get_date();
	let (_, second_month, second_year) = second.date.get_date();

	let mut d_month;
	let mut d_year;

	if (second_month as i32 - first_month as i32) < 0 {
		d_month = 1;
		d_year = 0;
	} else {
		d_month = second_month as u32 - first_month as u32;
		d_year = second_year - first_year;
	}

	let d_day: u32 = d_month*31 + d_year*31*12;
	let mut d_vol = second.vol - first.vol;
	d_vol.mean_vol(d_day as f64);

	(d_vol,	d_day)
}

fn calc_line(info: &Vec<TestInfo>) -> Vec<TestInfo> {
	
	if info.len() < 2 {
		panic!("For interpolation vector must have greater than 1 elements");
	}

	let mut ret_vol: Vec<TestInfo> = Vec::new();

	for i in 0..info.len() - 1 {
		let (step, days) = calc_step(&info[i], &info[i+1]);
		let mut now = info[i].date;
		let mut vol = info[i].vol;
		for _ in 0..days {
			ret_vol.push(TestInfo{
				date: now,
				vol: vol,
			});
			vol += step;
			now.add_day();
		}
	}
	ret_vol
}

pub fn interpolation(
	info: &Vec<TestInfo>, 
	method: MethodInter, 
	) -> Vec<TestInfo> {

	let ret_vol: Vec<TestInfo> = Vec::new();

	match method {
		MethodInter::Line => {
			calc_line(info)
		},
		MethodInter::Poly3 => {
			ret_vol
		},
	}
}