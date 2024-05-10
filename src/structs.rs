use std::collections::{HashSet,HashMap};

pub struct YellowLetters {
    pub data: HashMap<String, HashSet<u8>>,
}

pub struct GreenLetters {
    pub data: String,
}

pub struct GreyLetters {
    pub data: HashSet<String>,
}