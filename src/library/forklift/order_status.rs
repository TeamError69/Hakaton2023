use serde::{Serialize, Deserialize};
use regex::Regex;

pub const ORDER_STATUS_REGEX: (&str, &str, &str, &str) = (
	"started the task #(?<order_id>[0-9]{1,})",
	"reach the point K(?<point>[0-9]{1,})",
	"reach the target X(?<target>[0-9]{1,})",
	"finished the task #(?<order_id>[0-9]{1,})",
);

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatus {
	Started(u32),
	ReachPoint(u32),
	ReachTarget(u32),
	Finished(u32),
}

impl OrderStatus {
	pub fn order_id(&self) -> Result<Option<u32>, Box<dyn std::error::Error>> {
		match self {
			OrderStatus::Started(order_id) => Ok(Some(*order_id)),
			OrderStatus::ReachPoint(_) => Err("It's point".into()),
			OrderStatus::ReachTarget(_) => Err("It's target".into()),
			OrderStatus::Finished(order_id) => Ok(None),
		}
	}
	pub fn point(&self) -> Result<u32, Box<dyn std::error::Error>> {
		match self {
			OrderStatus::Started(_) => Err("It's order id".into()),
			OrderStatus::ReachPoint(point) => Ok(*point),
			OrderStatus::ReachTarget(_) => Err("It's target".into()),
			OrderStatus::Finished(_) => Err("It's order id".into()),
		}
	}
	pub fn target(&self) -> Result<u32, Box<dyn std::error::Error>> {
		match self {
			OrderStatus::Started(_) => Err("It's order id".into()),
			OrderStatus::ReachPoint(_) => Err("It's point".into()),
			OrderStatus::ReachTarget(target) => Ok(*target),
			OrderStatus::Finished(_) => Err("It's order id".into()),
		}
	}
}

fn parse(line: &str) -> Result<OrderStatus, Box<dyn std::error::Error>> {
	let __r = (
		Regex::new(ORDER_STATUS_REGEX.0)?,
		Regex::new(ORDER_STATUS_REGEX.1)?,
		Regex::new(ORDER_STATUS_REGEX.2)?,
		Regex::new(ORDER_STATUS_REGEX.3)?,
	);
	match (
	__r.0.captures(line),
	__r.1.captures(line),
	__r.2.captures(line),
	__r.3.captures(line),
	) {
		(Some(data), _, _, _) => Ok(OrderStatus::Started(data["order_id"].parse::<u32>()?)),
		(_, Some(data), _, _) => Ok(OrderStatus::ReachPoint(data["point"].parse::<u32>()?)),
		(_, _, Some(data), _) => Ok(OrderStatus::ReachTarget(data["target"].parse::<u32>()?)),
		(_, _, _, Some(data)) => Ok(OrderStatus::Finished(data["order_id"].parse::<u32>()?)),
		_ => Err(format!("{:?} is not status", line).into())
	}
}

impl TryFrom<&str> for OrderStatus {
	type Error = Box<dyn std::error::Error>;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		parse(value)
	}
}
