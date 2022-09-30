pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let div: i32 = x / y;
    let mods: i32 = x % y;
    return (div, mods)
}