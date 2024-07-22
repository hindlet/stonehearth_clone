use bevy::{math::Vec2, prelude::Resource};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use super::{inverse_lerp, lerp, OctaveNoiseGen};



#[derive(Resource)]
pub struct SelectiveNoiseGen {
    noise_one: OctaveNoiseGen,
    noise_two: OctaveNoiseGen,
    blend_noise: OctaveNoiseGen,
    _seed: u64,
    height_scale: f32,
}


impl SelectiveNoiseGen {
    pub fn new(
        seed: u64,

        num_octaves_one: u32,
        frequency_scale_one: f32,
        amplitude_decay_one: f32,
        noise_scale_one: f32,

        num_octaves_two: u32,
        frequency_scale_two: f32,
        amplitude_decay_two: f32,
        noise_scale_two: f32,

        blend_noise_scale: f32,
        height_scale: f32,
    ) -> Self{
        let mut rng = ChaChaRng::seed_from_u64(seed);

        let sub_seeds: (u64, u64, u64) = rng.gen();

        SelectiveNoiseGen {
            _seed: seed,
            height_scale,
            noise_one: OctaveNoiseGen::new(
                sub_seeds.0,
                num_octaves_one,
                frequency_scale_one,
                amplitude_decay_one,
                noise_scale_one,
                1.0
            ),
            noise_two: OctaveNoiseGen::new(
                sub_seeds.1,
                num_octaves_two,
                frequency_scale_two,
                amplitude_decay_two,
                noise_scale_two,
                1.0
            ),
            blend_noise: OctaveNoiseGen::new(
                sub_seeds.2,
                1,
                1.0,
                1.0,
                blend_noise_scale,
                1.0
            )
        }
    }

    pub fn sample(&self, pos: Vec2) -> f32 {

        let a = self.noise_one.sample(pos);
        let b = self.noise_two.sample(pos);
        let c = self.blend_noise.sample(pos);

        lerp(a, b, (c + 1.0) / 2.0) * self.height_scale
    }

    /// by default results are between -1 and 1
    pub fn gen_map(
        &self,
        size: [usize; 2],
        pos_offset: Vec2,
        vertex_step: f32,
        locally_normalised: bool,
        scale_zero_to_one: bool
    ) -> Vec<f32> {
        let mut output = Vec::new();

        let (mut min, mut max) = (f32::MAX, f32::MIN);

        for y in 0..size[1] {
            for x in 0..size[0] {
                let pos = Vec2::new(x as f32, y as f32) * vertex_step + pos_offset;
                let val = if scale_zero_to_one {
                    (self.sample(pos) / self.height_scale + 1.0) / 2.0 * self.height_scale
                } else {
                    self.sample(pos)
                };
                if val > max {max = val}
                if val < min {min = val}
                output.push(val)
            }
        }

        if locally_normalised {
            for val in output.iter_mut() {
                *val = inverse_lerp(min, max, *val);
            }
        }

        output
    }
    

}