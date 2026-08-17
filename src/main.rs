use uuidgen::{UUID, UUIDError};
use chrono::prelude::*;

fn main() -> Result<(), UUIDError>{
    let uuid = UUID::new()?.uuid();
    println!("\"_id\": \"{}\",", &uuid); 
    
    let today = Local::now();
    let day = today.day() as usize; 
    let month = today.month() as usize; 
    let year = today.year() as usize; 

    let date: timestampepoch::Date = timestampepoch::Date::new(day, month, year);
    let timestamp = date.timestamp(); 
    match timestamp {
        Some(value) => println!("\"payment_date\": {},", value),
        None               => println!("Timestamp is not available!"),
   }

    Ok(())
}
