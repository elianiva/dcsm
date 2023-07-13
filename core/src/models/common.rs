use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub enum ListOrDict {
    List(Vec<String>),
    Dict(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum StringOrList {
    String(String),
    List(Vec<String>),
}
