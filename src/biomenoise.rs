use crate::{
    biomes::Dimension,
    noise::{octave_init, OctaveNoise, PerlinNoise},
    utils::{set_seed, skip_next_n},
};

struct SurfaceNoise {
    xz_scale: f64,
    y_scale: f64,
    xz_factor: f64,
    y_factor: f64,
    octmin: OctaveNoise,
    octmax: OctaveNoise,
    octmain: OctaveNoise,
    octsurf: OctaveNoise,
    octdepth: OctaveNoise,
    oct: Vec<PerlinNoise>,
}

fn init_surface_noise(sn: &mut SurfaceNoise, dim: i32, seed: u64) {
    let mut s: u64 = 0;
    set_seed(&mut s, &seed);
    octave_init(&mut sn.octmin, &mut s, &mut sn.oct[0..16].to_vec(), -15, 16);
    octave_init(
        &mut sn.octmax,
        &mut s,
        &mut sn.oct[16..32].to_vec(),
        -15,
        16,
    );
    octave_init(&mut sn.octmain, &mut s, &mut sn.oct[32..].to_vec(), -7, 8);

    if dim == Dimension::DimEnd as i32 {
        sn.xz_scale = 2.0;
        sn.y_scale = 1.0;
        sn.xz_factor = 80.0;
        sn.y_factor = 160.0;
    } else
    // DIM_OVERWORLD
    {
        octave_init(&mut sn.octsurf, &mut s, &mut sn.oct[40..].to_vec(), -3, 4);
        skip_next_n(&mut s, 262 * 10);
        octave_init(
            &mut sn.octdepth,
            &mut s,
            &mut sn.oct[44..].to_vec(),
            -15,
            16,
        );

        sn.xz_scale = 0.9999999814507745;
        sn.y_scale = 0.9999999814507745;
        sn.xz_factor = 80.0;
        sn.y_factor = 160.0;
    }
}
