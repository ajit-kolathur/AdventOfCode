use std::{
    vec::Vec,
};

pub fn vector_str_to_int(vector: Vec<String>) -> Vec<i16> {
    return vector.into_iter().map(|x| x.parse::<i16>().unwrap()).collect();
}