#![allow(dead_code, unused_variables)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineData {
    prev_point: Point,
    current_point: Point,
    color: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    event: String,
    data: LineData
}