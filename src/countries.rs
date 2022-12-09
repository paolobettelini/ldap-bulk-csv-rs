use celes::Country;
use core::str::FromStr;

pub fn get_alpha_2(country: &str) -> Option<&str> {
    let result = Country::from_str(country);
    
    if let Ok(country) = result {
        Some(country.alpha2)
    } else {
        None
    }

}
