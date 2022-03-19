/// Creates a [`Point`] from the given coordinates.
///
/// ```txt
/// point!(<x_number>, <y_number>)
/// point! { x: <number>, y: <number> }
/// point!(<coordinate>)
/// ```
///
/// # Examples
///
/// Creating a [`Point`], supplying x/y values:
///
/// ```
/// use geo_types::point;
///
/// let p = point!(181.2, 51.79);
///
/// assert_eq!(p.x(), 181.2);
/// assert_eq!(p.y(), 51.79);
/// ```
///
/// [`Point`]: ./struct.Point.html
#[macro_export]
macro_rules! point {
    ( $($tag:tt : $val:expr),* $(,)? ) => {
        $crate::point! ( $crate::coord! { $( $tag: $val , )* } )
    };
    ( $x:expr, $y:expr $(,)? ) => {
        $crate::point! { x: $x, y: $y }
    };
    ( $coord:expr $(,)? ) => {
        $crate::PointTZM($coord)
    };
}

/// Creates a [`Coordinate`] from the given scalars.
///
/// ```txt
/// coord! { x: <number>, y: <number> }
/// ```
///
/// # Examples
///
/// Creating a [`Coordinate`], supplying x/y values:
///
/// ```
/// use geo_types::coord;
///
/// let c = coord! { x: 181.2, y: 51.79 };
///
/// assert_eq!(c, geo_types::coord! { x: 181.2, y: 51.79 });
/// ```
///
/// [`Coordinate`]: ./struct.Point.html
#[macro_export]
macro_rules! coord {
    (x: $x:expr, y: $y:expr $(,)* ) => {
        $crate::CoordTZM::new(
            $x,
            $y,
            $crate::NoValue::default(),
            $crate::NoValue::default(),
        )
    };
    (x: $x:expr, y: $y:expr, z: $z:expr $(,)* ) => {
        $crate::CoordTZM::new($x, $y, $z, $crate::NoValue::default())
    };
    (x: $x:expr, y: $y:expr, m: $m:expr $(,)* ) => {
        $crate::CoordTZM::new($x, $y, $crate::NoValue::default(), $m)
    };
    (x: $x:expr, y: $y:expr, z: $z:expr, m: $m:expr $(,)* ) => {
        $crate::CoordTZM::new($x, $y, $z, $m)
    };
}

/// Creates a [`LineString`] containing the given coordinates.
///
/// ```txt
/// line_string![Coordinate OR (x: <number>, y: <number>), …]
/// ```
///
/// # Examples
///
/// Creating a [`LineString`], supplying x/y values:
///
/// ```
/// use geo_types::line_string;
///
/// let ls = line_string![
///     (x: -21.95156, y: 64.1446),
///     (x: -21.951, y: 64.14479),
///     (x: -21.95044, y: 64.14527),
///     (x: -21.951445, y: 64.145508),
/// ];
///
/// assert_eq!(ls[1], geo_types::coord! {
///     x: -21.951,
///     y: 64.14479
/// });
/// ```
///
/// Creating a [`LineString`], supplying [`Coordinate`]s:
///
/// ```
/// use geo_types::line_string;
///
/// let coord1 = geo_types::coord! {
///     x: -21.95156,
///     y: 64.1446,
/// };
/// let coord2 = geo_types::coord! {
///     x: -21.951,
///     y: 64.14479,
/// };
/// let coord3 = geo_types::coord! {
///     x: -21.95044,
///     y: 64.14527,
/// };
/// let coord4 = geo_types::coord! {
///     x: -21.951445,
///     y: 64.145508,
/// };
///
/// let ls = line_string![coord1, coord2, coord3, coord4];
///
/// assert_eq!(
///     ls[1],
///     geo_types::coord! {
///         x: -21.951,
///         y: 64.14479
///     }
/// );
/// ```
///
/// [`Coordinate`]: ./struct.Coordinate.html
/// [`LineString`]: ./line_string/struct.LineString.html
#[macro_export]
macro_rules! line_string {
    () => { $crate::LineStringTZM::new(vec![]) };
    (
        $(( $($tag:tt : $val:expr),* $(,)? )),*
        $(,)?
    ) => {
        line_string![
            $(
                $crate::coord! { $( $tag: $val , )* },
            )*
        ]
    };
    (
        $($coord:expr),*
        $(,)?
    ) => {
        $crate::LineStringTZM::new(
            <[_]>::into_vec(
                ::std::boxed::Box::new(
                    [$($coord), *]
                )
            )
        )
    };
}

/// Creates a [`Polygon`] containing the given coordinates.
///
/// ```txt
/// polygon![Coordinate OR (x: <number>, y: <number>), …]
///
/// // or
///
/// polygon!(
///     exterior: [Coordinate OR (x: <number>, y: <number>), …],
///     interiors: [
///         [Coordinate OR (x: <number>, y: <number>), …],
///         …
///     ],
/// )
/// ```
///
/// # Examples
///
/// Creating a [`Polygon`] without interior rings, supplying x/y values:
///
/// ```
/// use geo_types::polygon;
///
/// let poly = polygon![
///     (x: -111., y: 45.),
///     (x: -111., y: 41.),
///     (x: -104., y: 41.),
///     (x: -104., y: 45.),
/// ];
///
/// assert_eq!(
///     poly.exterior()[1],
///     geo_types::coord! { x: -111., y: 41. },
/// );
/// ```
///
/// Creating a [`Polygon`], supplying x/y values:
///
/// ```
/// use geo_types::polygon;
///
/// let poly = polygon!(
///     exterior: [
///         (x: -111., y: 45.),
///         (x: -111., y: 41.),
///         (x: -104., y: 41.),
///         (x: -104., y: 45.),
///     ],
///     interiors: [
///         [
///             (x: -110., y: 44.),
///             (x: -110., y: 42.),
///             (x: -105., y: 42.),
///             (x: -105., y: 44.),
///         ],
///     ],
/// );
///
/// assert_eq!(
///     poly.exterior()[1],
///     geo_types::coord! { x: -111., y: 41. },
/// );
/// ```
///
/// [`Coordinate`]: ./struct.Coordinate.html
/// [`Polygon`]: ./struct.Polygon.html
#[macro_export]
macro_rules! polygon {
    () => { $crate::PolygonTZM::new(line_string![], vec![]) };
    (
        exterior: [
            $(( $($exterior_tag:tt : $exterior_val:expr),* $(,)? )),*
            $(,)?
        ],
        interiors: [
            $([
                $(( $($interior_tag:tt : $interior_val:expr),* $(,)? )),*
                $(,)?
            ]),*
            $(,)?
        ]
        $(,)?
    ) => {
        polygon!(
            exterior: [
                $(
                    $crate::coord! { $( $exterior_tag: $exterior_val , )* },
                )*
            ],
            interiors: [
                $([
                    $($crate::coord! { $( $interior_tag: $interior_val , )* }),*
                ]),*
            ],
        )
    };
    (
        exterior: [
            $($exterior_coord:expr),*
            $(,)?
        ],
        interiors: [
            $([
                $($interior_coord:expr),*
                $(,)?
            ]),*
            $(,)?
        ]
        $(,)?
    ) => {
        $crate::PolygonTZM::new(
            $crate::line_string![
                $($exterior_coord), *
            ],
            <[_]>::into_vec(
                ::std::boxed::Box::new(
                    [
                        $(
                            $crate::line_string![$($interior_coord),*]
                        ), *
                    ]
                )
            )
        )
    };
    (
        $(( $($tag:tt : $val:expr),* $(,)? )),*
        $(,)?
    ) => {
        polygon![
            $($crate::coord! { $( $tag: $val , )* }),*
        ]
    };
    (
        $($coord:expr),*
        $(,)?
    ) => {
        $crate::PolygonTZM::new(
            $crate::line_string![$($coord,)*],
            vec![],
        )
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_point() {
        let p = point!(x: 1.2, y: 3.4);
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);

        let p = point! {
            x: 1.2,
            y: 3.4,
        };
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);

        let p = point!(1.2, 3.4);
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);

        let p = point!(1.2, 3.4,);
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);

        let p = point!(coord! { x: 1.2, y: 3.4 });
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);

        let p = point!(coord! { x: 1.2, y: 3.4 },);
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);
    }

    #[test]
    fn test_line() {
        let ls = line_string![(x: -1.2f32, y: 3.4f32)];
        assert_eq!(ls[0], coord! { x: -1.2, y: 3.4 });

        let ls = line_string![
            (x: -1.2f32, y: 3.4f32),
        ];
        assert_eq!(ls[0], coord! { x: -1.2, y: 3.4 });

        let ls = line_string![(
            x: -1.2f32,
            y: 3.4f32,
        )];
        assert_eq!(ls[0], coord! { x: -1.2, y: 3.4 });

        let ls = line_string![
            (x: -1.2f32, y: 3.4f32),
            (x: -5.6, y: 7.8),
        ];
        assert_eq!(ls[0], coord! { x: -1.2, y: 3.4 });
        assert_eq!(ls[1], coord! { x: -5.6, y: 7.8 });
    }

    #[test]
    fn test_polygon() {
        let p = polygon!(
            exterior: [(x: 1, y: 2)],
            interiors: [[(x: 3, y: 4)]]
        );
        assert_eq!(p.exterior()[0], coord! { x: 1, y: 2 });
        assert_eq!(p.interiors()[0][0], coord! { x: 3, y: 4 });

        let p = polygon!(
            exterior: [(x: 1, y: 2)],
            interiors: [[(x: 3, y: 4)]],
        );
        assert_eq!(p.exterior()[0], coord! { x: 1, y: 2 });
        assert_eq!(p.interiors()[0][0], coord! { x: 3, y: 4 });

        let p = polygon!(
            exterior: [(x: 1, y: 2, )],
            interiors: [[(x: 3, y: 4, )]],
        );
        assert_eq!(p.exterior()[0], coord! { x: 1, y: 2 });
        assert_eq!(p.interiors()[0][0], coord! { x: 3, y: 4 });

        let p = polygon!(
            exterior: [(x: 1, y: 2, ), ],
            interiors: [[(x: 3, y: 4, ), ]],
        );
        assert_eq!(p.exterior()[0], coord! { x: 1, y: 2 });
        assert_eq!(p.interiors()[0][0], coord! { x: 3, y: 4 });

        let p = polygon!(
            exterior: [(x: 1, y: 2, ), ],
            interiors: [[(x: 3, y: 4, ), ], ],
        );
        assert_eq!(p.exterior()[0], coord! { x: 1, y: 2 });
        assert_eq!(p.interiors()[0][0], coord! { x: 3, y: 4 });
    }
}
