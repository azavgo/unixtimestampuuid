use uuidv4::*;
use chrono::prelude::*;
use system_helpers::*;

//Custom error
#[derive(Debug)]
pub enum UnixTimestampUUIDError { 
    UUIDError(uuidv4::UUIDError),
    SystemHelpersError(system_helpers::SystemHelpersError),   
}

impl From<uuidv4::UUIDError> for UnixTimestampUUIDError {
    fn from(error: uuidv4::UUIDError) -> Self {
        UnixTimestampUUIDError::UUIDError(error)
    }
}

impl From<system_helpers::SystemHelpersError> for UnixTimestampUUIDError {
    fn from(error: system_helpers::SystemHelpersError) -> Self {
        UnixTimestampUUIDError::SystemHelpersError(error)
    }
}
fn main() -> Result<(), UnixTimestampUUIDError> {
    let uuid = uuid()?;
    //println!("\"_id\": \"{}\",", &uuid); 
    
    let today = Local::now();
    let day = today.day() as usize; 
    let month = today.month() as usize; 
    let year = today.year() as usize; 

    let date: timestampepoch::Date = timestampepoch::Date::new(day, month, year);
    let timestamp_option = date.timestamp(); 
    match timestamp_option {
        Some(value) => {
                                let timestamp = value.to_string(); 
                                //println!("\"payment_date\": {},", &timestamp);
                                let output = format!("  \"_id\": \"{}\",\n  \"payment_date\": {},", &uuid, &timestamp);
                                println!("{}", &output);
                                copy_to_clipboard(output)?;
                              },
        None               => eprintln!("Timestamp is not available!"),
   }

    Ok(())
}
