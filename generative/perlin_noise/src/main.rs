// A 2D perlin noise generator

struct Vector {
    x: f64,
    y: f64,
}


impl Vector {

    fn new() -> Vector {
        return Vector {
            x: 0.0,
            y: 0.0,
        }
    }

    fn dot(&self, other: Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    fn add(&self, other: Vector) -> Vector {
        return Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn element_wise_product(&self, other: Vector) -> Vector {
        return Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

}

fn generate_2d_perlin_noise(size: (usize, usize)) -> Vec<Vec<Vector>> {
    /*
    Generate a NxM 2D grid of perlin noise
    */

    let mut grid: Vec<Vec<Vector>> = vec![vec![Vector::new(), size.0]; size.1];
    grid

    // generate random unit vectors
}

fn main() {
    println!("Hello, world!");
}
