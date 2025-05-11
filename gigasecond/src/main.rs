use time::{OffsetDateTime, PrimitiveDateTime as DateTime};
use time::Duration;

fn main()
{
    let now_odt = OffsetDateTime::now_utc();
    println!("Date after One billion seconds: {}", after(DateTime::new(now_odt.date(),now_odt.time())));
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let billion = 1_000_000_000;
    start + Duration::seconds(billion)
}
