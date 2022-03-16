extern crate geo;

use geo_types::{point, GenPoint};

fn main() {
    let p = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let GenPoint(coord) = p;
    println!("Point at ({}, {})", coord.x, coord.y);
}
