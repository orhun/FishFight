// Perlin Stuff
// Ported from https://github.com/josephg/noisejs/blob/master/perlin.js
// This is all magic to me, don't ask me anything about it

pub struct NoiseGenerator {
    grad_p: [(i32, i32, i32); 512],
    perm: [usize; 512],
}

impl NoiseGenerator {
    const P: [i32; 256] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
        69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
        252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
        168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
        211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
        216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
        164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
        213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
        253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
        192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
        138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
    ];

    const GRAD_3: [(i32, i32, i32); 12] = [
        (1, 1, 0),
        (-1, 1, 0),
        (1, -1, 0),
        (-1, -1, 0),
        (1, 0, 1),
        (-1, 0, 1),
        (1, 0, -1),
        (-1, 0, -1),
        (0, 1, 1),
        (0, -1, 1),
        (0, 1, -1),
        (0, -1, -1),
    ];

    pub fn new(seed: i32) -> NoiseGenerator {
        let mut n = NoiseGenerator {
            grad_p: [(0, 0, 0); 512],
            perm: [0; 512],
        };
        n.seed(seed);

        n
    }

    pub fn seed(&mut self, seed: i32) {
        let mut seed = seed;
        if seed > 0 && seed < 1 {
            // Scale the seed out
            seed *= 65536;
        }

        if seed < 256 {
            seed |= seed << 8;
        }

        for i in 0..256 {
            let v = if i & 1 > 0 {
                NoiseGenerator::P[i] ^ (seed & 255)
            } else {
                NoiseGenerator::P[i] ^ ((seed >> 8) & 255)
            };

            //self.gradP[i] =
            self.perm[i] = v as usize;
            self.perm[i + 256] = v as usize;

            self.grad_p[i + 256] = NoiseGenerator::GRAD_3[(v % 12) as usize];
            self.grad_p[i] = self.grad_p[i + 256];
            //noise_generator::gradP[i] = this::gradP[i + 256] = gradv
        }
    }

    pub fn perlin_2d(&mut self, x: f32, y: f32) -> f32 {
        // Generates values from -.5 to .5
        let mut x_f = x.floor() as i32;
        let mut y_f = y.floor() as i32;

        let x = x - x_f as f32;
        let y = y - y_f as f32;

        x_f &= 255;
        y_f &= 255;

        let n00 = NoiseGenerator::dot2(self.grad_p[x_f as usize + self.perm[y_f as usize]], x, y);
        let n01 = NoiseGenerator::dot2(
            self.grad_p[x_f as usize + self.perm[(y_f + 1) as usize]],
            x,
            y - 1.0,
        );
        let n10 = NoiseGenerator::dot2(
            self.grad_p[(x_f + 1) as usize + self.perm[y_f as usize]],
            x - 1.0,
            y,
        );
        let n11 = NoiseGenerator::dot2(
            self.grad_p[(x_f + 1) as usize + self.perm[(y_f + 1) as usize]],
            x - 1.0,
            y - 1.0,
        );

        let u = NoiseGenerator::fade(x);

        NoiseGenerator::lerp(
            NoiseGenerator::lerp(n00, n10, u),
            NoiseGenerator::lerp(n01, n11, u),
            NoiseGenerator::fade(y),
        )
    }

    fn dot2(tuple: (i32, i32, i32), x: f32, y: f32) -> f32 {
        tuple.0 as f32 * x + tuple.1 as f32 * y
    }

    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6. - 15.) + 10.)
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        (1. - t) * a + t * b
    }
}
