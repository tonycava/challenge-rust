use std::f64::consts::PI;

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let first = (other.x + self.x).powi(2);
        let second = (other.y + self.y).powi(2);
        return (first + second).sqrt();
    }
}


impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: {
                Point {
                    x,
                    y,
                }
            },
            radius,
        }
    }
    pub fn area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        let d = ((self.center.x - other.center.x) * (self.center.x - other.center.x)
            + (self.center.y - other.center.y) * (self.center.y - other.center.y)).sqrt();

        if d <= self.radius - other.radius {
            return true;
        } else if d <= other.radius - self.radius {
            return true;
        } else if d < self.radius + other.radius {
            return true;
        } else if d == self.radius + other.radius {
            return true;
        }

        return false;
    }
}
