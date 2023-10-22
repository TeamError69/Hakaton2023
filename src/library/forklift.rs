mod status_forklift;
mod order_status;

use serde::{Serialize, Deserialize};

use regex::Regex;
use chrono::{
	NaiveDateTime,
	FixedOffset,
	Duration,
};

pub use status_forklift::StatusForklift;
pub use order_status::OrderStatus;


// const REGEX: &str = r"Forklift #(?<forklift>[0-9]{1,}) of warehouse #(?<warehouse>[0-9]{1,}) have (?<task_type>[0-9a-zA-Z/ +//#+/]{1,}) #(?<order_id>[0-9]{1,}) at (?<time>[\d\-\:/ +//.+/]{1,})$";

pub const FORKLIFT_REGEX: &str = r"Forklift #(?<forklift>[0-9]{1,}) of warehouse #(?<warehouse>[0-9]{1,}) have (?<order_status>[0-9a-zA-Z/ +//#+/]{1,}) at (?<time>[\d\-\:/ +//.+/]{1,})$";

#[derive(Serialize, Deserialize, Debug)]
pub struct Forklift {
	warehouse: u32,
	id: u32,
	order_id: Option<u32>,
	status: StatusForklift,
}

impl Forklift {
	pub fn new(warehouse: u32, id: u32, order_id: Option<u32>, status: StatusForklift) -> Forklift {
		Forklift {
			warehouse,
			id,
			order_id,
			status,
		}
	}
	pub fn warehouse(&self) -> u32 {
		self.warehouse
	}
	pub fn id(&self) -> u32 {
		self.id
	}
	pub fn try_update(&mut self, data: Self) -> Result<(), Box<dyn std::error::Error>> {
		if self.warehouse != data.warehouse {
			return Err("Update error: warehouses differ".into());
		};
		if self.id != data.id {
			return Err("Update error: forklift differ".into());
		}
		match (&self.status.status, &data.status.status) {
			(&OrderStatus::Finished(_), &OrderStatus::Started(__el)) => {
				self.order_id = Some(__el);
				self.status = data.status;
			},
			(&OrderStatus::Started(start_id), &OrderStatus::Finished(finish_id)) => {
				if finish_id != start_id {
					return Err("It is forbidden to stop Different tasks".into());
				};
				self.order_id = None;
				self.status = data.status;
			},
			(_, &OrderStatus::Started(__el)) => {
				self.order_id = Some(__el);
				self.status = data.status;
			},
			_ => self.status = data.status,
		}
		Ok(())
	}
}

fn parse(line: &str) -> Result<Forklift, Box<dyn std::error::Error>> {
	let __r = Regex::new(FORKLIFT_REGEX)?;
	if let Some(data) = __r.captures(line) {
		let order_status = OrderStatus::try_from(&data["order_status"])?;
		let _id = order_status.order_id().unwrap_or_else(|_| None);
		return Ok(Forklift::new(
			data["warehouse"].parse::<u32>()?,
			data["forklift"].parse::<u32>()?,
			_id,
			StatusForklift::new(
				NaiveDateTime::parse_from_str(&data["time"], "%Y-%m-%d %H:%M:%S%.6f")?,
				order_status,
			),
		));
	};
	Err(format!("{:?} is not a forklift", line).into())
}

impl TryFrom<&str> for Forklift {
	type Error = Box<dyn std::error::Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(parse(value)?)
	}
}

impl PartialEq<Self> for Forklift {
	fn eq(&self, other: &Self) -> bool {
		(self.warehouse == other.warehouse) && (self.id == other.id)
	}
}


