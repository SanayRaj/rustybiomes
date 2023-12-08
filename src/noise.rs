use crate::utils::{next_double, next_int, skip_next_n};

#[derive(Clone)]
pub struct PerlinNoise {
    d: [u8; 512],
    a: f64,
    b: f64,
    c: f64,
    amplitude: f64,
    lacunarity: f64,
}

pub struct OctaveNoise {
    octcnt: usize,
    octaves: Vec<PerlinNoise>,
}

fn perlin_init(noise: &mut PerlinNoise, seed: &mut u64) {
    // memset(noise, 0, sizeof(*noise));
    noise.a = next_double(seed) * 256.0;
    noise.b = next_double(seed) * 256.0;
    noise.c = next_double(seed) * 256.0;
    noise.amplitude = 1.0;
    noise.lacunarity = 1.0;

    for i in 0..256 {
        noise.d[i] = i as u8;
    }

    for i in 0..256 {
        let j = next_int(seed, 256 - i) as u8 + i as u8;
        let n = noise.d[i as usize];
        noise.d[i as usize] = noise.d[j as usize];
        noise.d[j as usize] = n;
        noise.d[(i + 256) as usize] = noise.d[i as usize];
    }
}

pub fn octave_init(
    noise: &mut OctaveNoise,
    seed: &mut u64,
    octaves: &mut Vec<PerlinNoise>,
    omin: i32,
    len: usize,
) {
    let end = omin + len as i32 - 1;
    let mut persist = 1.0 / ((1u64 << len) as f64 - 1.0);
    let mut lacuna = 2.0f64.powi(end);

    if len < 1 || end > 0 {
        println!("octaveInit(): unsupported octave range");
        return;
    }

    let mut i;

    if end == 0 {
        perlin_init(&mut octaves[0], seed);
        octaves[0].amplitude = persist;
        octaves[0].lacunarity = lacuna;
        let mut persist = persist * 2.0;
        let mut lacuna = lacuna * 0.5;
        i = 1;
    } else {
        skip_next_n(seed, (end as u64) * 262);
        i = 0;
    }

    while i < len {
        perlin_init(&mut octaves[i], seed);
        octaves[i].amplitude = persist;
        octaves[i].lacunarity = lacuna;
        persist *= 2.0;
        lacuna *= 0.5;
        i += 1;
    }

    noise.octaves = octaves.clone();
    noise.octcnt = len;
}
