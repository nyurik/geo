use super::Contains;
use crate::*;
use geo_types::{GenCoord, GenPoint, Measure, ZCoord};

// ┌────────────────────────────────┐
// │ Implementations for Point      │
// └────────────────────────────────┘

impl<T: CoordNum, Z: ZCoord, M: Measure> Contains<GenCoord<T, Z, M>> for GenPoint<T, Z, M> {
    fn contains(&self, coord: &GenCoord<T, Z, M>) -> bool {
        &self.0 == coord
    }
}

impl<T: CoordNum, Z: ZCoord, M: Measure> Contains<GenPoint<T, Z, M>> for GenPoint<T, Z, M> {
    fn contains(&self, p: &GenPoint<T, Z, M>) -> bool {
        self.contains(&p.0)
    }
}

// ┌────────────────────────────────┐
// │ Implementations for MultiPoint │
// └────────────────────────────────┘
impl<G, T> Contains<G> for MultiPoint<T>
where
    T: CoordNum,
    Point<T>: Contains<G>,
{
    fn contains(&self, rhs: &G) -> bool {
        self.iter().any(|p| p.contains(rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use geo_types::point;

    #[test]
    fn test_point_contains() {
        assert!(point!(x: 1_i32, y: 2).contains(&point!(x: 1, y: 2)));
        assert!(point!(x: 1.0_f32, y: 2.).contains(&point!(x: 1., y: 2.)));

        assert!(point!(x: 1, y: 2, z: 3).contains(&point!(x: 1, y: 2, z: 3)));
        assert!(point!(x: 1, y: 2, m: 4).contains(&point!(x: 1, y: 2, m: 4)));
        assert!(point!(x: 1, y: 2, z: 3, m: 4).contains(&point!(x: 1, y: 2, z: 3, m: 4)));
    }
}
