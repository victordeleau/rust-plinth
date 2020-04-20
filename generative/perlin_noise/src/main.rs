// A 2D perlin noise generator
// inspired from => https://flafla2.github.io/2014/08/09/perlinnoise.html

use std::{thread, time};

use ansi_escapes::*; // specific characters

//////////////////////////////////////////////////////////////////////////////

pub struct PerlinGen {
    hash_table: [u64; 256],
    repeat: f64,
}

impl PerlinGen {

    pub fn new(repeat: u64) -> PerlinGen {

        return PerlinGen {
            hash_table: [151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180],

            repeat: repeat as f64,
        };
    }


    fn get_hash(&self, value: u64) -> u64 {
        return self.hash_table[(value % 255) as usize];
    }

    fn linear_interpolation(&self, v1: f64, v2: f64, weight: f64) -> f64 {
        ((1. - weight) * v1) + (weight * v2)
    }

    fn fade(&self,v: f64) -> f64 {
        v * v * v * (v * (v * 6. - 15.) + 10.)
    }

    fn inc(&self, mut v: u64) -> u64 {
        v += 1;
        v %= self.repeat as u64;
        v
    }

    
    fn grad(&self, hash: u64, x: f64, y: f64, z: f64) -> f64 {

        match hash & 0xF{
            0x0 => return x + y,
            0x1 => return -x + y,
            0x2 => return x - y,
            0x3 => return -x - y,
            0x4 => return x + z,
            0x5 => return -x + z,
            0x6 => return x - z,
            0x7 => return -x - z,
            0x8 => return y + z,
            0x9 => return -y + z,
            0xA => return y - z,
            0xB => return -y - z,
            0xC => return y + x,
            0xD => return -y + z,
            0xE => return y - x,
            0xF => return -y - z,
            _ => return 0., // impossible
        }
    }


    pub fn for_point(&self, mut x: f64, mut y: f64, mut z: f64) -> Result<f64, &'static str> {
        /*
        Return Perlin noise value for provided point.
        */

        x = x % self.repeat;
        y = y % self.repeat;
        z = z % self.repeat;

        // get box corner coordinates
        let x0: u64 = (x as u64) & 255;
        let y0: u64 = (y as u64) & 255;
        let z0: u64 = (z as u64) & 255;

        // get point coordinates in box
        let x_box: f64 = x - (x0 as f64);
        let y_box: f64 = y - (y0 as f64);
        let z_box: f64 = z - (z0 as f64);

        // ease point
        let u: f64 = self.fade(x_box);
        let v: f64 = self.fade(y_box);
        let w: f64 = self.fade(z_box);

        // hash box coordinates
        let aaa = self.get_hash(
            self.get_hash(
                self.get_hash(x0)+y0)+z0);

		let aba = self.get_hash(
            self.get_hash(
                self.get_hash(x0)+self.inc(y0))+z0);

		let aab = self.get_hash(
            self.get_hash(
                self.get_hash(x0)+y0)+self.inc(z0));

		let abb = self.get_hash(
            self.get_hash(
                self.get_hash(x0)+self.inc(y0))+self.inc(z0));

		let baa = self.get_hash(
            self.get_hash(
                self.get_hash(self.inc(x0))+y0)+z0);

		let bba = self.get_hash(
            self.get_hash(
                self.get_hash(self.inc(x0))+self.inc(y0))+z0);

		let bab = self.get_hash(
            self.get_hash(
                self.get_hash(self.inc(x0))+y0)+self.inc(z0));

		let bbb = self.get_hash(
            self.get_hash(
                self.get_hash(self.inc(x0))+self.inc(y0))+self.inc(z0));

        //println!("{} {} {} {}", aa, ab, ba, bb);

        // compute dot product between distance and field vectors
        let mut d0 = self.linear_interpolation(
            self.grad(aaa, x_box, y_box, z_box),
            self.grad(baa, x_box-1., y_box, z_box),
            u);

        let mut d1 = self.linear_interpolation(
            self.grad(aba, x_box, y_box-1., z_box),
            self.grad(bba, x_box-1., y_box-1., z_box),
            u);

        let e0 = self.linear_interpolation(d0, d1, v);

        d0 = self.linear_interpolation(
            self.grad(aab, x_box, y_box, z_box-1.),
            self.grad(bab, x_box-1., y_box, z_box-1.),
            u);

        d1 = self.linear_interpolation(
            self.grad(abb, x_box, y_box-1., z_box-1.),
            self.grad(bbb, x_box-1., y_box-1., z_box-1.),
            u);

        let e1 = self.linear_interpolation(d0, d1, v);

        Ok( (self.linear_interpolation(e0, e1, w)+1.)/2. )
    }
}


fn main() {

    println!("Perlin Noise generator !\n");

    let perlin_generator: PerlinGen = PerlinGen::new(16);

    let mut value: f64;

    let fps: f64 = 10.;
    let delay = time::Duration::from_millis((1.0/fps * 1000.0) as u64);

    let low: u64 = 0;
    let high: u64 = 50;

    let size: f64 = 10.;
    let mut depth: f64 = 0.;

    loop {

        thread::sleep(delay);

        depth += 0.1;
        depth %= 50.;

        print!("      ");
        for _ in 0..50+4 { print!("\u{2588}\u{2588}"); }
        print!("\n      \u{2588}\u{2588}                                                                                                        \u{2588}\u{2588}\n");

        for i in low..high {

            print!("      \u{2588}\u{2588}  ");

            for j in low..high {

                // get point value
                value = perlin_generator.for_point(
                    i as f64/size,
                    j as f64/size,
                    depth as f64/size
                ).unwrap();

                // print value to console
                if value < 0.5 {
                    print!("\u{2588}\u{2588}");
                } else {
                    print!("  ");
                }
            }

            println!("  \u{2588}\u{2588}");
        }

        print!("      \u{2588}\u{2588}                                                                                                        \u{2588}\u{2588}\n");
        print!("      ");
        for _ in 0..50+4 { print!("\u{2588}\u{2588}"); }
        println!("\n");

        for _ in 0..50+5 { print!("{}{}", CursorUp(1), EraseLine); }

    }
}