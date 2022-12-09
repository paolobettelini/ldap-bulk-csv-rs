use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "fullName")]
    #[serde(with = "serde_bytes")]
    pub full_name: Vec<u8>,
    #[serde(rename = "firstname")]
    #[serde(with = "serde_bytes")]
    pub first_name: Vec<u8>,
    #[serde(rename = "middleInitial")]
    #[serde(with = "serde_bytes")]
    pub middle_initial: Vec<u8>,
    #[serde(rename = "lastname")]
    #[serde(with = "serde_bytes")]
    pub last_name: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub username: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub email: Vec<u8>,
    #[serde(rename = "streetaddress")]
    #[serde(with = "serde_bytes")]
    pub street_address: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub city: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub zipcode: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub state: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub country: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub department: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub password: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub telephone: Vec<u8>,
    #[serde(rename = "jobtitle")]
    #[serde(with = "serde_bytes")]
    pub job_title: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub company: Vec<u8>,
    #[serde(rename = "Oupath")]
    #[serde(with = "serde_bytes")]
    pub oupath: Vec<u8>,
}
