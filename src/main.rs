use uuidgen::{UUID, UUIDError};
use timestampepoch::*;

fn main() -> Result<(), UUIDError>{
    let uuid = UUID::new()?.uuid();
    println!("\"_id\": \"{}\",", &uuid); 
    
    let date: Date = Date::new(17, 8, 2026);
    let timestamp = date.timestamp(); 
    match timestamp {
        Some(value) => println!("\"payment_date\": {},", value),
        None               => println!("Timestamp is not available!"),
   }

    Ok(())
}
