use crate::{
    CoordNum, Error, GeometryCollection, GeometryCollectionTZM, Line, LineString, LineStringTZM,
    LineTZM, Measure, MultiLineString, MultiLineStringTZM, MultiPoint, MultiPointTZM, MultiPolygon,
    MultiPolygonTZM, NoValue, Point, PointTZM, Polygon, PolygonTZM, Rect, RectTZM, Triangle,
    TriangleTZM, ZCoord,
};

#[cfg(any(feature = "approx", test))]
use approx::{AbsDiffEq, RelativeEq};
use core::any::type_name;
use std::convert::TryFrom;

/// An enum representing any possible geometry type.
///
/// All `Geo` types can be converted to a `Geometry` member using `.into()` (as part of the
/// `std::convert::Into` pattern), and `Geo` types implement the `TryFrom` trait in order to
/// convert _back_ from enum members.
///
/// # Example
///
/// ```
/// use std::convert::TryFrom;
/// use geo_types::{Point, point, Geometry, GeometryCollection};
/// let p = point!(x: 1.0, y: 1.0);
/// let pe: Geometry<f64> = p.into();
/// let pn = Point::try_from(pe).unwrap();
/// ```
///
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GeometryTZM<T: CoordNum, Z: ZCoord, M: Measure> {
    Point(PointTZM<T, Z, M>),
    Line(LineTZM<T, Z, M>),
    LineString(LineStringTZM<T, Z, M>),
    Polygon(PolygonTZM<T, Z, M>),
    MultiPoint(MultiPointTZM<T, Z, M>),
    MultiLineString(MultiLineStringTZM<T, Z, M>),
    MultiPolygon(MultiPolygonTZM<T, Z, M>),
    GeometryCollection(GeometryCollectionTZM<T, Z, M>),
    Rect(RectTZM<T, Z, M>),
    Triangle(TriangleTZM<T, Z, M>),
}

pub type Geometry<T> = GeometryTZM<T, NoValue, NoValue>;
pub type GeometryM<T, M> = GeometryTZM<T, NoValue, M>;
pub type GeometryZ<T> = GeometryTZM<T, T, NoValue>;
pub type GeometryZM<T, M> = GeometryTZM<T, T, M>;

impl<T: CoordNum, Z: ZCoord, M: Measure> From<PointTZM<T, Z, M>> for GeometryTZM<T, Z, M> {
    fn from(x: PointTZM<T, Z, M>) -> Self {
        Self::Point(x)
    }
}
impl<T: CoordNum, Z: ZCoord, M: Measure> From<LineTZM<T, Z, M>> for GeometryTZM<T, Z, M> {
    fn from(x: LineTZM<T, Z, M>) -> Self {
        Self::Line(x)
    }
}
impl<T: CoordNum, Z: ZCoord, M: Measure> From<LineStringTZM<T, Z, M>> for GeometryTZM<T, Z, M> {
    fn from(x: LineStringTZM<T, Z, M>) -> Self {
        Self::LineString(x)
    }
}
impl<T: CoordNum, Z: ZCoord, M: Measure> From<PolygonTZM<T, Z, M>> for GeometryTZM<T, Z, M> {
    fn from(x: PolygonTZM<T, Z, M>) -> Self {
        Self::Polygon(x)
    }
}
impl<T: CoordNum, Z: ZCoord, M: Measure> From<MultiPointTZM<T, Z, M>> for GeometryTZM<T, Z, M> {
    fn from(x: MultiPointTZM<T, Z, M>) -> Self {
        Self::MultiPoint(x)
    }
}
impl<T: CoordNum, Z: ZCoord, M: Measure> From<MultiLineStringTZM<T, Z, M>>
    for GeometryTZM<T, Z, M>
{
    fn from(x: MultiLineStringTZM<T, Z, M>) -> Self {
        Self::MultiLineString(x)
    }
}
impl<T: CoordNum, Z: ZCoord, M: Measure> From<MultiPolygonTZM<T, Z, M>> for GeometryTZM<T, Z, M> {
    fn from(x: MultiPolygonTZM<T, Z, M>) -> Self {
        Self::MultiPolygon(x)
    }
}

impl<T: CoordNum> From<Rect<T>> for Geometry<T> {
    fn from(x: Rect<T>) -> Self {
        Self::Rect(x)
    }
}

impl<T: CoordNum> From<Triangle<T>> for Geometry<T> {
    fn from(x: Triangle<T>) -> Self {
        Self::Triangle(x)
    }
}

macro_rules! try_from_geometry_impl {
    ($($type: ident),+) => {
        $(
        /// Convert a Geometry enum into its inner type.
        ///
        /// Fails if the enum case does not match the type you are trying to convert it to.
        impl <T: CoordNum> TryFrom<Geometry<T>> for $type<T> {
            type Error = Error;

            fn try_from(geom: Geometry<T>) -> Result<Self, Self::Error> {
                match geom {
                    Geometry::$type(g) => Ok(g),
                    other => Err(Error::MismatchedGeometry {
                        expected: type_name::<$type<T>>(),
                        found: inner_type_name(other)
                    })
                }
            }
        }
        )+
    }
}

try_from_geometry_impl!(
    Point,
    Line,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    Rect,
    Triangle
);

fn inner_type_name<T>(geometry: Geometry<T>) -> &'static str
where
    T: CoordNum,
{
    match geometry {
        Geometry::Point(_) => type_name::<Point<T>>(),
        Geometry::Line(_) => type_name::<Line<T>>(),
        Geometry::LineString(_) => type_name::<LineString<T>>(),
        Geometry::Polygon(_) => type_name::<Polygon<T>>(),
        Geometry::MultiPoint(_) => type_name::<MultiPoint<T>>(),
        Geometry::MultiLineString(_) => type_name::<MultiLineString<T>>(),
        Geometry::MultiPolygon(_) => type_name::<MultiPolygon<T>>(),
        Geometry::GeometryCollection(_) => type_name::<GeometryCollection<T>>(),
        Geometry::Rect(_) => type_name::<Rect<T>>(),
        Geometry::Triangle(_) => type_name::<Triangle<T>>(),
    }
}

#[cfg(any(feature = "approx", test))]
impl<T> RelativeEq for Geometry<T>
where
    T: AbsDiffEq<Epsilon = T> + CoordNum + RelativeEq,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    /// Equality assertion within a relative limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{Geometry, polygon};
    ///
    /// let a: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7., y: 9.), (x: 0., y: 0.)].into();
    /// let b: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7.01, y: 9.), (x: 0., y: 0.)].into();
    ///
    /// approx::assert_relative_eq!(a, b, max_relative=0.1);
    /// approx::assert_relative_ne!(a, b, max_relative=0.001);
    /// ```
    ///
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        match (self, other) {
            (Geometry::Point(g1), Geometry::Point(g2)) => g1.relative_eq(g2, epsilon, max_relative),
            (Geometry::Line(g1), Geometry::Line(g2)) => g1.relative_eq(g2, epsilon, max_relative),
            (Geometry::LineString(g1), Geometry::LineString(g2)) => {
                g1.relative_eq(g2, epsilon, max_relative)
            }
            (Geometry::Polygon(g1), Geometry::Polygon(g2)) => {
                g1.relative_eq(g2, epsilon, max_relative)
            }
            (Geometry::MultiPoint(g1), Geometry::MultiPoint(g2)) => {
                g1.relative_eq(g2, epsilon, max_relative)
            }
            (Geometry::MultiLineString(g1), Geometry::MultiLineString(g2)) => {
                g1.relative_eq(g2, epsilon, max_relative)
            }
            (Geometry::MultiPolygon(g1), Geometry::MultiPolygon(g2)) => {
                g1.relative_eq(g2, epsilon, max_relative)
            }
            (Geometry::GeometryCollection(g1), Geometry::GeometryCollection(g2)) => {
                g1.relative_eq(g2, epsilon, max_relative)
            }
            (Geometry::Rect(g1), Geometry::Rect(g2)) => g1.relative_eq(g2, epsilon, max_relative),
            (Geometry::Triangle(g1), Geometry::Triangle(g2)) => {
                g1.relative_eq(g2, epsilon, max_relative)
            }
            (_, _) => false,
        }
    }
}

#[cfg(any(feature = "approx", test))]
impl<T: AbsDiffEq<Epsilon = T> + CoordNum> AbsDiffEq for Geometry<T> {
    type Epsilon = T;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    /// Equality assertion with an absolute limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::{Geometry, polygon};
    ///
    /// let a: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7., y: 9.), (x: 0., y: 0.)].into();
    /// let b: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7.01, y: 9.), (x: 0., y: 0.)].into();
    ///
    /// approx::assert_abs_diff_eq!(a, b, epsilon=0.1);
    /// approx::assert_abs_diff_ne!(a, b, epsilon=0.001);
    /// ```
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (self, other) {
            (Geometry::Point(g1), Geometry::Point(g2)) => g1.abs_diff_eq(g2, epsilon),
            (Geometry::Line(g1), Geometry::Line(g2)) => g1.abs_diff_eq(g2, epsilon),
            (Geometry::LineString(g1), Geometry::LineString(g2)) => g1.abs_diff_eq(g2, epsilon),
            (Geometry::Polygon(g1), Geometry::Polygon(g2)) => g1.abs_diff_eq(g2, epsilon),
            (Geometry::MultiPoint(g1), Geometry::MultiPoint(g2)) => g1.abs_diff_eq(g2, epsilon),
            (Geometry::MultiLineString(g1), Geometry::MultiLineString(g2)) => {
                g1.abs_diff_eq(g2, epsilon)
            }
            (Geometry::MultiPolygon(g1), Geometry::MultiPolygon(g2)) => g1.abs_diff_eq(g2, epsilon),
            (Geometry::GeometryCollection(g1), Geometry::GeometryCollection(g2)) => {
                g1.abs_diff_eq(g2, epsilon)
            }
            (Geometry::Rect(g1), Geometry::Rect(g2)) => g1.abs_diff_eq(g2, epsilon),
            (Geometry::Triangle(g1), Geometry::Triangle(g2)) => g1.abs_diff_eq(g2, epsilon),
            (_, _) => false,
        }
    }
}
