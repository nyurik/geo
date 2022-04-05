use geo::algorithm::relate::IntersectionMatrix;
use geo::{coord, Geometry, NoValue, Point};
use serde::{Deserialize, Deserializer};

use super::Result;

/// Example of the XML that these structures represent
///
/// ```xml
/// <run>
/// <precisionModel scale="1.0" offsetx="0.0" offsety="0.0"/>
///
/// <case>
/// <desc>AA disjoint</desc>
/// <a>
/// POLYGON(
/// (0 0, 80 0, 80 80, 0 80, 0 0))
/// </a>
/// <b>
/// POLYGON(
/// (100 200, 100 140, 180 140, 180 200, 100 200))
/// </b>
/// <test><op name="relate" arg3="FF2FF1212" arg1="A" arg2="B"> true </op>
/// </test>
/// <test>  <op name="intersects" arg1="A" arg2="B">   false   </op></test>
/// <test>  <op name="contains" arg1="A" arg2="B">   false   </op></test>
/// </case>
/// </run>
/// ```
#[derive(Debug, Deserialize)]
pub(crate) struct Run {
    #[serde(rename = "case")]
    pub cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Case {
    #[serde(default)]
    pub(crate) desc: String,

    #[serde(deserialize_with = "deserialize_from_wkt")]
    pub(crate) a: Geometry<f64>,

    #[serde(deserialize_with = "deserialize_opt_geometry", default)]
    pub(crate) b: Option<Geometry<f64>>,

    #[serde(rename = "test", default)]
    pub(crate) tests: Vec<Test>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Test {
    #[serde(rename = "op")]
    pub(crate) operation_input: OperationInput,
}

#[derive(Debug, Deserialize)]
pub struct CentroidInput {
    pub(crate) arg1: String,

    #[serde(rename = "$value", deserialize_with = "deserialize_point_from_wkt")]
    pub(crate) expected: Option<geo::Point<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct ConvexHullInput {
    pub(crate) arg1: String,

    #[serde(rename = "$value", deserialize_with = "deserialize_from_wkt")]
    pub(crate) expected: geo::Geometry<f64>,
}

#[derive(Debug, Deserialize)]
pub struct IntersectsInput {
    pub(crate) arg1: String,
    pub(crate) arg2: String,

    #[serde(rename = "$value", deserialize_with = "deserialize_from_str")]
    pub(crate) expected: bool,
}

#[derive(Debug, Deserialize)]
pub struct RelateInput {
    pub(crate) arg1: String,
    pub(crate) arg2: String,

    #[serde(rename = "arg3", deserialize_with = "deserialize_from_str")]
    pub(crate) expected: IntersectionMatrix,
}

#[derive(Debug, Deserialize)]
pub struct ContainsInput {
    pub(crate) arg1: String,
    pub(crate) arg2: String,

    #[serde(rename = "$value", deserialize_with = "deserialize_from_str")]
    pub(crate) expected: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name")]
pub(crate) enum OperationInput {
    #[serde(rename = "getCentroid")]
    CentroidInput(CentroidInput),

    #[serde(rename = "convexhull")]
    ConvexHullInput(ConvexHullInput),

    #[serde(rename = "intersects")]
    IntersectsInput(IntersectsInput),

    #[serde(rename = "relate")]
    RelateInput(RelateInput),

    #[serde(rename = "contains")]
    ContainsInput(ContainsInput),

    #[serde(other)]
    Unsupported,
}

#[derive(Debug)]
pub(crate) enum Operation {
    Centroid {
        subject: Geometry<f64>,
        expected: Option<Point<f64>>,
    },
    Contains {
        subject: Geometry<f64>,
        target: Geometry<f64>,
        expected: bool,
    },
    ConvexHull {
        subject: Geometry<f64>,
        expected: Geometry<f64>,
    },
    Intersects {
        subject: Geometry<f64>,
        clip: Geometry<f64>,
        expected: bool,
    },
    Relate {
        a: Geometry<f64>,
        b: Geometry<f64>,
        expected: IntersectionMatrix,
    },
}

impl OperationInput {
    pub(crate) fn into_operation(self, case: &Case) -> Result<Operation> {
        let geometry = &case.a;
        match self {
            Self::CentroidInput(centroid_input) => {
                assert_eq!("A", centroid_input.arg1);
                Ok(Operation::Centroid {
                    subject: geometry.clone(),
                    expected: centroid_input.expected,
                })
            }
            Self::ConvexHullInput(convex_hull_input) => {
                assert_eq!("A", convex_hull_input.arg1);
                Ok(Operation::ConvexHull {
                    subject: geometry.clone(),
                    expected: convex_hull_input.expected,
                })
            }
            Self::IntersectsInput(input) => {
                assert_eq!("A", input.arg1);
                assert_eq!("B", input.arg2);
                assert!(
                    case.b.is_some(),
                    "intersects test case must contain geometry b"
                );
                Ok(Operation::Intersects {
                    subject: geometry.clone(),
                    clip: case.b.clone().expect("no geometry b in case"),
                    expected: input.expected,
                })
            }
            Self::RelateInput(input) => {
                assert_eq!("A", input.arg1);
                assert_eq!("B", input.arg2);
                assert!(
                    case.b.is_some(),
                    "intersects test case must contain geometry b"
                );
                Ok(Operation::Relate {
                    a: geometry.clone(),
                    b: case.b.clone().expect("no geometry b in case"),
                    expected: input.expected,
                })
            }
            Self::ContainsInput(input) => {
                assert_eq!("A", input.arg1);
                assert_eq!("B", input.arg2);
                assert!(
                    case.b.is_some(),
                    "intersects test case must contain geometry b"
                );
                Ok(Operation::Contains {
                    subject: geometry.clone(),
                    target: case.b.clone().expect("no geometry b in case"),
                    expected: input.expected,
                })
            }
            Self::Unsupported => Err("This OperationInput not supported".into()),
        }
    }
}

pub fn deserialize_opt_geometry<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Geometry<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_from_wkt")] Geometry<f64>);

    Option::<Wrapper>::deserialize(deserializer).map(|opt_wrapped| opt_wrapped.map(|w| w.0))
}

pub fn deserialize_from_str<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    T: std::str::FromStr,
    D: Deserializer<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    String::deserialize(deserializer)
        .and_then(|str| T::from_str(&str).map_err(serde::de::Error::custom))
}

pub fn deserialize_point_from_wkt<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Point<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(wkt::deserialize_point::<D, f64>(deserializer)?.map(to_point))
}

pub fn deserialize_from_wkt<'de, D>(deserializer: D) -> std::result::Result<Geometry<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    return Ok(convert(wkt::deserialize_geometry::<D, f64>(deserializer)?));
}

fn convert(itm: geo_types::Geometry<f64>) -> geo::GeometryTZM<f64, NoValue, NoValue> {
    match itm {
        geo_types::Geometry::Point(v) => geo::Geometry::Point(Point::<f64>::new(v.x(), v.y())),
        geo_types::Geometry::LineString(v) => geo::Geometry::LineString(to_line_string(v)),
        geo_types::Geometry::Polygon(v) => geo::Geometry::Polygon(to_polygon(v)),
        geo_types::Geometry::MultiPoint(geo_types::MultiPoint(v)) => {
            geo::Geometry::MultiPoint(geo::MultiPoint::new(v.into_iter().map(to_point).collect()))
        }
        geo_types::Geometry::MultiLineString(geo_types::MultiLineString(v)) => {
            geo::Geometry::MultiLineString(geo::MultiLineString::new(
                v.into_iter().map(to_line_string).collect(),
            ))
        }
        geo_types::Geometry::MultiPolygon(geo_types::MultiPolygon(v)) => {
            geo::Geometry::MultiPolygon(geo::MultiPolygon::new(
                v.into_iter().map(to_polygon).collect(),
            ))
        }
        geo_types::Geometry::GeometryCollection(geo_types::GeometryCollection(v)) => {
            geo::Geometry::GeometryCollection(geo::GeometryCollection::new_from(
                v.into_iter().map(convert).collect(),
            ))
        }
        geo_types::Geometry::Line(v) => todo!("{v:?}"),
        geo_types::Geometry::Rect(v) => todo!("{v:?}"),
        geo_types::Geometry::Triangle(v) => todo!("{v:?}"),
    }
}

fn to_point(v: geo_types::Point<f64>) -> geo::Point<f64> {
    Point::new(v.x(), v.y())
}

fn to_polygon(v: geo_types::Polygon<f64>) -> geo::PolygonTZM<f64, NoValue, NoValue> {
    let (exterior, interiors) = v.into_inner();
    geo::Polygon::new(
        to_line_string(exterior),
        interiors.into_iter().map(to_line_string).collect(),
    )
}

fn to_line_string(v: geo_types::LineString<f64>) -> geo::LineStringTZM<f64, NoValue, NoValue> {
    let coords =
        v.0.into_iter()
            .map(|coord| coord! { x: coord.x, y: coord.y })
            .collect();
    geo::LineString::new(coords)
}
