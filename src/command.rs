extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm {key: String},
}