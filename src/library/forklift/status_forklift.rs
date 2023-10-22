use serde::{Serialize, Deserialize};

use regex::Regex;
use chrono::{
	NaiveDateTime,
	FixedOffset,
	Duration,
};

use super::order_status::OrderStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusForklift {
	time: NaiveDateTime,
	pub status: OrderStatus,
}

impl StatusForklift {
	pub fn new(time: NaiveDateTime, status: OrderStatus) -> StatusForklift {
		StatusForklift {
			time,
			status,
		}
	}
}
