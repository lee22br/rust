use time::PrimitiveDateTime as DateTime;
use time::Duration;

fn main()
{
    println!("Date after One billion seconds: {}", after(Date::now()));
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let billion = 1000000000;
    start + Duration::seconds(billion)
}
