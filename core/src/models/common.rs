use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub enum ListOrDict {
    List(Vec<String>),
    Dict(HashMap<String, String>),
}

pub enum StringOrList {
    String(String),
    List(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StringOrNumber<T = i32> {
    String(String),
    Number(T),
}
