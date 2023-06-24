use std::collections::HashMap;

pub enum ListOrDict {
    List(Vec<String>),
    Dict(HashMap<String, String>),
}

pub enum StringOrList {
    String(String),
    List(Vec<String>),
}

pub enum StringOrNumber<T> {
    String(String),
    Number(T),
}
