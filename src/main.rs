use uuidgen::{UUID, UUIDError};
use timestampepoch::*;

fn main() -> Result<(), UUIDError>{
    let uuid = UUID::new()?.uuid();
    println!("\"_id\": \"{}\",", &uuid); 
    
    let date: Date = Date::new(17, 8, 2026);
    println!("\"payment_date\": {},", date.timestamp().unwrap());

    Ok(())
}
