pub use does_it_fit::*;

fn main() {
    // println!(
    //     "Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
    //     area_fit(2, 4, GeometricalShapes::Rectangle, 100, 2, 1) // false
    // );
    // println!(
    //     "Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
    //     area_fit(5, 5, GeometricalShapes::Triangle, 3, 5, 3) // true
    // );
    // println!(
    //     "Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
    //     volume_fit(5, 5, 5, GeometricalVolumes::Sphere, 3, 2, 0, 0) // true
    // );
    // println!(
    //     "Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
    //     volume_fit(5, 7, 5, GeometricalVolumes::Parallelepiped, 1, 6, 7, 4) // true
    // );
    println!("{}", volume_fit(2, 5, 3 , GeometricalVolumes::Sphere, 1, 1, 1, 1))
}