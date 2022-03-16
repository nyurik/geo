extern crate geo;

use geo_types::{GenericPoint, point};

fn main() {
    let p = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let GenericPoint(coord) = p;
    println!("Point at ({}, {})", coord.x, coord.y);
}
