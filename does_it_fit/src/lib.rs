mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    println!("{x}");
    println!("{y}");
    println!("{:?}", objects);
    println!("{times}");
    println!("{a}");
    println!("{b}");

    return match objects {
        GeometricalShapes::Square => {
            let rect = rectangle_area(x, y);
            let area = square_area(a);
            if area <= rect * times {
                return true;
            }
            false
        }
        GeometricalShapes::Circle => {
            let rect = rectangle_area(x, y);
            let area = circle_area(a);
            if area >= rect as f64 * times as f64 {
                return true;
            }
            false
        }
        GeometricalShapes::Rectangle => {
            let rect = rectangle_area(x, y);
            let area = rectangle_area(a, b);
            if area >= rect * times {
                return true;
            }
            false
        }
        GeometricalShapes::Triangle => {
            let rect = rectangle_area(x, y);
            let area = triangle_area(a, b);
            if area <= rect as f64 * times as f64 {
                return true;
            }
            false
        }
    };
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    false
}