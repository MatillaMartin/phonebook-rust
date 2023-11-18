use serde::{Serialize, Deserialize};
#[derive(Clone, Serialize, Deserialize)]
pub struct Contact
{
    pub name: String,
    pub email: String,
    pub address: String,
    pub mobile: String,
}
