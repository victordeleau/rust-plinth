// A 2D perlin noise generator

use rand::Rng;

use ansi_escapes::*; // specific characters


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

impl Clone for Vector {
    fn clone(&self) -> Vector {
        return Vector {
            x: self.x,
            y: self.y,
        }
    }
}


//////////////////////////////////////////////////////////////////////////////

pub struct PerlinGen {
    hash_table: [u64; 256],
}

impl PerlinGen {

    pub fn new() -> PerlinGen {

        return PerlinGen {
            hash_table: [151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180],
        };
    }


    fn get_hash(&self, value: u64) -> u64 {
        return self.hash_table[(value % 255) as usize];
    }


    fn linear_interpolation(&self, v1: f64, v2: f64, weight: f64) -> f64 {
        ((1. - weight) * v1) + (weight * v2)
    }

    
    fn grad(hash: u64, x: f64, y: f64) -> f64 {

        match hash & 0b11 {
            0x0 => return x + y,
            0x1 => return -x + y,
            0x2 => return x - y,
            0x3 => return -x - y,
            _ => return 0., // impossible
        }
    }


    pub fn for_point(&self, x: f64, y: f64) -> Result<f64, &'static str> {

        // get box corner coordinates
        let x0: u64 = x.floor() as u64;
        let x1: u64 = x.ceil() as u64;
        let y0: u64 = y.floor() as u64;
        let y1: u64 = y.ceil() as u64;

        // get point coordinates in box
        let x_box: f64 = x - x0 as f64;
        let y_box: f64 = y - y0 as f64;

        // computes corners to point vectors
        let v0: Vector = Vector { x: x_box-x0 as f64, y: y_box-y1 as f64 };
        let v1: Vector = Vector { x: x_box-x1 as f64, y: y_box-y1 as f64 };
        let v2: Vector = Vector { x: x_box-x0 as f64, y: y_box-y0 as f64 };
        let v3: Vector = Vector { x: x_box-x1 as f64, y: y_box-y0 as f64 };

        // hash box coordinates
        let aa: u64 = self.get_hash( self.get_hash( x0) + y0 );
        let ab: u64 = self.get_hash( self.get_hash( x0) + y1 );
        let ba: u64 = self.get_hash( self.get_hash( x1) + y0 );
        let bb: u64 = self.get_hash( self.get_hash( x1) + y1 );

        // compute dot product between distance and field vectors
        let d0 = self.linear_interpolation(
            Self::grad(aa, x_box, y_box),
            Self::grad(ba, x_box, y_box),
            x_box);

        let d1 = self.linear_interpolation(
            Self::grad(aa, x_box, y_box),
            Self::grad(ba, x_box, y_box),
            x_box);

        Ok( self.linear_interpolation(d0, d1, y_box) )
    }
}


fn main() {

    println!("Perlin Noise generator !");

    let perlin_generator: PerlinGen = PerlinGen::new();

    let mut value: f64;
    for i in 0..50 {
        for j in 0..50 {
            value = perlin_generator.for_point(i as f64/100., j as f64/100.).unwrap();

            if value < 0.1 {
                print!("\u{2588}\u{2588}");
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}