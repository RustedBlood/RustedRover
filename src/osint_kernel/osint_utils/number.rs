use phonenumber::{PhoneNumber, country};

pub fn get_num_info(number: &str) -> Result<PhoneNumber, phonenumber::ParseError> {
    let region: Option<country::Id> = Some(country::RU);

    let parsed_num = phonenumber::parse(region, number)?;

    Ok(parsed_num)
}
