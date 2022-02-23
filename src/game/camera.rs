use macroquad::prelude::*;
use macroquad::rand::gen_range;

use core::noise::NoiseGenerator;

struct Shake {
    direction: (f32, f32),
    kind: ShakeType,
    magnitude: f32,
    length: f32, //in frames, but stored in float to avoid casting
    age: f32,
    random_offset: f32,
    frequency: f32, // 1 is pretty standard, .2 is a punch (with 10 frames of shake it oscillates about max twice). With .5 it's more of a rumble
}

#[allow(dead_code)]
enum ShakeType {
    Noise,
    Sinusoidal,
    Rotational,
}

pub struct GameCamera {
    bounds: Rect,
    follow_buffer: Vec<(Vec2, f32)>,
    shake: Vec<Shake>,
    noisegen: NoiseGenerator,
    noisegen_position: f32,

    pub manual: Option<(Vec2, f32)>,
    player_rects: Vec<Rect>,
}

impl GameCamera {
    const BUFFER_CAPACITY: usize = 20;

    pub fn new(map_size: Vec2) -> GameCamera {
        let bounds = Rect::new(0.0, 0.0, map_size.x, map_size.y);

        GameCamera {
            bounds,
            follow_buffer: vec![],
            shake: vec![],
            manual: None,
            noisegen: NoiseGenerator::new(5),
            noisegen_position: 5.0,
            player_rects: Vec::new(),
        }
    }

    pub fn add_player_rect(&mut self, rect: Rect) {
        self.player_rects.push(rect);
    }
}

#[allow(dead_code)]
impl GameCamera {
    pub fn shake_noise(&mut self, magnitude: f32, length: i32, frequency: f32) {
        self.shake.push(Shake {
            direction: (1.0, 1.0),
            kind: ShakeType::Noise,
            magnitude,
            length: length as f32,
            age: 0.0,
            random_offset: rand::gen_range(1.0, 100.0),
            frequency,
        });
    }

    pub fn shake_noise_dir(
        &mut self,
        magnitude: f32,
        length: i32,
        frequency: f32,
        direction: (f32, f32),
    ) {
        self.shake.push(Shake {
            direction,
            kind: ShakeType::Noise,
            magnitude,
            length: length as f32,
            age: 0.0,
            random_offset: rand::gen_range(1.0, 100.0),
            frequency,
        });
    }

    pub fn shake_sinusoidal(&mut self, magnitude: f32, length: i32, frequency: f32, angle: f32) {
        self.shake.push(Shake {
            direction: (angle.cos(), angle.sin()),
            kind: ShakeType::Sinusoidal,
            magnitude,
            length: length as f32,
            age: 0.0,
            random_offset: 0.0,
            frequency,
        });
    }

    pub fn shake_rotational(&mut self, magnitude: f32, length: i32) {
        self.shake.push(Shake {
            direction: (1.0, 1.0),
            kind: ShakeType::Rotational,
            magnitude: magnitude * (gen_range(0, 2) as f32 - 0.5) * 2.0,
            length: length as f32,
            age: 0.0,
            random_offset: 0.0,
            frequency: 0.0,
        });
    }

    pub fn get_shake(&mut self) -> (Vec2, f32) {
        //(x translate, y translate, rotation)
        self.noisegen_position += 0.5;
        let mut shake_offset = vec2(0.0, 0.0);
        let mut shake_rotation = 0.0;
        for i in 0..self.shake.len() {
            let strength = 1.0 - self.shake[i].age / self.shake[i].length;
            match self.shake[i].kind {
                ShakeType::Noise => {
                    shake_offset.x += self.noisegen.perlin_2d(
                        self.noisegen_position * self.shake[i].frequency
                            + self.shake[i].random_offset,
                        5.0,
                    ) * self.shake[i].magnitude
                        * self.shake[i].direction.0
                        * strength
                        * 100.0;
                    shake_offset.y += self.noisegen.perlin_2d(
                        self.noisegen_position * self.shake[i].frequency
                            + self.shake[i].random_offset,
                        7.0,
                    ) * self.shake[i].magnitude
                        * self.shake[i].direction.1
                        * strength
                        * 100.0;
                }
                ShakeType::Sinusoidal => {
                    shake_offset.x += (self.noisegen_position * self.shake[i].frequency * 1.0)
                        .sin()
                        * self.shake[i].magnitude
                        * self.shake[i].direction.0
                        * strength
                        * 50.0; // Noise values are +/- 0.5, trig is twice as large
                    shake_offset.y += (self.noisegen_position * self.shake[i].frequency * 1.0)
                        .sin()
                        * self.shake[i].magnitude
                        * self.shake[i].direction.1
                        * strength
                        * 50.0;
                }
                ShakeType::Rotational => {
                    //shake_rotation += self.noisegen.perlin_2d(self.noisegen_position * self.shake[i].frequency + self.shake[i].random_offset, 5.0) * self.shake[i].magnitude * strength.powi(3);
                    shake_rotation += self.shake[i].magnitude * strength.powi(3) * 3.0;
                }
            };

            self.shake[i].age += 1.0;
        }

        self.shake.retain(|s| s.age < s.length);

        shake_offset.x = (shake_offset.x.abs() + 1.0).log2() * shake_offset.x.signum(); // log2(x+1) is almost linear from 0-1, but then flattens out. Limits the screenshake so if there is lots at the same time, the scene won't fly away
        shake_offset.y = (shake_offset.y.abs() + 1.0).log2() * shake_offset.y.signum();

        (shake_offset, shake_rotation)
    }

    pub fn update(&mut self) {
        {
            let aspect = screen_width() / screen_height();

            let mut middle_point = vec2(0.0, 0.0);
            let mut min = vec2(10000.0, 10000.0);
            let mut max = vec2(-10000.0, -10000.0);

            let player_cnt = self.player_rects.len();
            for rect in self.player_rects.drain(0..player_cnt) {
                let camera_pox_middle = rect.point() + rect.size() / 2.0;
                //let k = if player.controller_id == 1 { 0.8 } else { 0.2 };
                middle_point += camera_pox_middle; // * k;

                min = min.min(camera_pox_middle);
                max = max.max(camera_pox_middle);
            }

            middle_point /= player_cnt as f32;

            let border_x = 150.0;
            let border_y = 200.0;
            let mut scale = (max - min).abs() + vec2(border_x * 2.0, border_y * 2.0);

            if scale.x > scale.y * aspect {
                scale.y = scale.x / aspect;
            }

            let mut zoom = scale.y;

            // bottom camera bound
            if scale.y / 2. + middle_point.y > self.bounds.h {
                middle_point.y = self.bounds.h - scale.y / 2.0;
            }

            if let Some((override_target, override_zoom)) = self.manual {
                middle_point = override_target;
                zoom = override_zoom;
            }

            self.follow_buffer.insert(0, (middle_point, zoom));
            self.follow_buffer.truncate(Self::BUFFER_CAPACITY);
        }
        let mut sum_pos = (0.0f64, 0.0f64);
        let mut sum_zoom = 0.0;
        for (pos, zoom) in &self.follow_buffer {
            sum_pos.0 += pos.x as f64;
            sum_pos.1 += pos.y as f64;
            sum_zoom += *zoom as f64;
        }
        let mut middle_point = vec2(
            (sum_pos.0 / self.follow_buffer.len() as f64) as f32,
            (sum_pos.1 / self.follow_buffer.len() as f64) as f32,
        );
        let zoom = (sum_zoom / self.follow_buffer.len() as f64) as f32;

        let shake = self.get_shake();
        middle_point += shake.0;
        let rotation = shake.1;

        let aspect = screen_width() / screen_height();

        // let middle_point = vec2(400.0, 600.0);
        // let zoom = 400.0;
        let macroquad_camera = Camera2D {
            target: middle_point,
            zoom: vec2(1. / aspect, -1.0) / zoom * 2.0,
            rotation,
            ..Camera2D::default()
        };

        scene::set_camera(0, Some(macroquad_camera));
    }
}
