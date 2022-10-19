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
    println!("area here !!!!!");
    println!("x: {x}");
    println!("y: {y}");
    println!("obj: {:?}", objects);
    println!("times: {times}");
    println!("a: {a}");
    println!("b: {b}");

    if times == 0 { return true }
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
            if area <= rect * times {
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
    println!("volume here !!!!!");
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
    println!("obj: {:?}", objects);
    println!("times: {times}");
    println!("a: {a}");
    println!("b: {b}");
    println!("c: {c}");
    if times == 0 { return true }
    return match objects {
        GeometricalVolumes::Cube => {
            let rect = rectangle_area(x, y);
            let area = cube_volume(a);
            if area <= rect * times {
                return true;
            }
            false
        }
        GeometricalVolumes::Sphere => {
            let rect = rectangle_area(x, y);
            let area = sphere_volume(a);
            if area >= rect as f64 * times as f64 {
                return true;
            }
            false
        }
        GeometricalVolumes::Cone => {
            let rect = rectangle_area(x, y);
            let area = cone_volume(a, b);
            if area <= (rect * times) as f64 {
                return true;
            }
            false
        }
        GeometricalVolumes::Pyramid => {
            let rect = rectangle_area(x, y);
            let area = triangular_pyramid_volume(a as f64, b);
            if area <= rect as f64 * times as f64 {
                return true;
            }
            false
        }
        GeometricalVolumes::Parallelepiped => {
            let rect = rectangle_area(x, y);
            let area = parallelepiped_volume(a, b, c);
            if area <= rect * times {
                return true;
            }
            false
        }
    };
}