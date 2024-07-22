use bevy::{math::Vec2, prelude::Resource};
use rand_chacha::{ChaChaRng, rand_core::SeedableRng};
use rand::Rng;
use super::{inverse_lerp, simplex_2d::simplex2d};



#[derive(Resource)]
pub struct OctaveNoiseGen {
    offsets: Vec<Vec2>,
    octaves: u32,
    _seed: u64,

    frequency_octave_scale: f32,
    amplitude_octave_decay: f32,
    noise_scale: f32,
    height_scale: f32,
}


impl OctaveNoiseGen {
    pub fn new(
        seed: u64,
        num_octaves: u32,
        frequency_scale: f32,
        amplitude_decay: f32,
        noise_scale: f32,
        height_scale: f32
    ) -> Self{
        let mut offsets = Vec::new();
        let mut rng = ChaChaRng::seed_from_u64(seed);

        for _i in 0..num_octaves {
            offsets.push(Vec2::new(
                rng.gen_range(-1000000..=100000) as f32,
                rng.gen_range(-100000..=100000) as f32,
            ))
        }

        OctaveNoiseGen {
            offsets,
            octaves: num_octaves,
            _seed: seed,
            noise_scale: noise_scale.max(0.0000001),
            frequency_octave_scale: frequency_scale.max(0.0),
            amplitude_octave_decay: amplitude_decay.clamp(0.0, 1.0),
            height_scale
        }
    }


    pub fn sample(&self, pos: Vec2) -> f32 {

        let (mut freq, mut ampl, mut total) = (1.0, 1.0, 0.0);

        for i in 0..self.octaves as usize {
            let x_sample = (pos.x + self.offsets[i].x) / self.noise_scale * freq;
            let y_sample = (pos.y + self.offsets[i].y) / self.noise_scale * freq;

            total += simplex2d(x_sample, y_sample) * ampl;
            freq *= self.frequency_octave_scale;
            ampl *= self.amplitude_octave_decay;
        }

        total * self.height_scale
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