//! Coherent mountain belts, foothills and continuous climate fields.

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub seed: u64,
    pub side: usize,
    pub cell_size: f64,
    pub sea_level: f64,
    pub relief: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            seed: 8675309,
            side: 257,
            cell_size: 128.0,
            sea_level: 63.0,
            relief: 1400.0,
        }
    }
}

impl Config {
    pub fn validate(self) -> Result<Self, &'static str> {
        if !(3..=1025).contains(&self.side) {
            return Err("side must be in 3..=1025");
        }
        if !self.cell_size.is_finite() || self.cell_size <= 0.0 {
            return Err("cell_size must be finite and positive");
        }
        if !self.sea_level.is_finite() || !self.relief.is_finite() || self.relief <= 0.0 {
            return Err("sea level must be finite and relief finite and positive");
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Climate {
    pub temperature_c: f64,
    pub rainfall_mm_year: f64,
    pub moisture: f64,
}

#[derive(Clone, Debug)]
pub struct HeightField {
    pub config: Config,
    pub ground: Vec<f64>,
    pub climate: Vec<Climate>,
}

impl HeightField {
    pub fn generate(config: Config) -> Result<Self, &'static str> {
        let config = config.validate()?;
        let n = config.side;
        let span = (n - 1) as f64 * config.cell_size;
        let mut ground = Vec::with_capacity(n * n);
        let mut climate = Vec::with_capacity(n * n);
        for z in 0..n {
            let mut upwind = 0.0_f64;
            for x in 0..n {
                let wx = x as f64 * config.cell_size - span / 2.0;
                let wz = z as f64 * config.cell_size - span / 2.0;
                let h = macro_height(wx, wz, span, config);
                // West-to-east prevailing wind. A decaying upstream ridge creates
                // a continuous rain shadow; no discrete biome boundary enters here.
                let elevation = (h - config.sea_level).max(0.0);
                let shadow = (upwind - elevation).max(0.0);
                let rain = (1200.0 + elevation * 0.38 - shadow * 0.65
                    + noise(wx / 4200.0, wz / 4200.0, config.seed ^ 91) * 300.0)
                    .clamp(180.0, 2400.0);
                climate.push(Climate {
                    temperature_c: 19.0 - elevation * 0.0065 - wz / span * 12.0,
                    rainfall_mm_year: rain,
                    moisture: ((rain - 180.0) / 2220.0).clamp(0.0, 1.0),
                });
                ground.push(h);
                upwind = (upwind * (-config.cell_size / 5500.0).exp()).max(elevation);
            }
        }
        Ok(Self {
            config,
            ground,
            climate,
        })
    }

    /// Useful for independent drainage fixtures and imported elevation rasters.
    pub fn from_elevations(config: Config, ground: Vec<f64>) -> Result<Self, &'static str> {
        let config = config.validate()?;
        if ground.len() != config.side * config.side || ground.iter().any(|x| !x.is_finite()) {
            return Err("elevation raster must have side squared finite values");
        }
        let climate = vec![
            Climate {
                temperature_c: 15.0,
                rainfall_mm_year: 1000.0,
                moisture: 0.5
            };
            ground.len()
        ];
        Ok(Self {
            config,
            ground,
            climate,
        })
    }

    pub fn point(&self, index: usize) -> [f64; 2] {
        let n = self.config.side;
        let half = (n - 1) as f64 * self.config.cell_size / 2.0;
        [
            (index % n) as f64 * self.config.cell_size - half,
            (index / n) as f64 * self.config.cell_size - half,
        ]
    }
}

fn macro_height(x: f64, z: f64, span: f64, c: Config) -> f64 {
    let u = x / (span * 0.5);
    let v = z / (span * 0.5);
    let warp = noise(x / 7000.0, z / 7000.0, c.seed) * 0.16;
    let edge = (u * u + v * v * 1.35).sqrt() + warp;
    let coast = smooth(((0.94 - edge) / 0.32).clamp(0.0, 1.0));
    // An elongated, curved uplift belt. Broad shoulders explicitly accompany
    // the crest; independent high-frequency spikes do not define the relief.
    let along = x * 0.78 + z * 0.63;
    let across = -x * 0.63
        + z * 0.78
        + 1100.0 * (along / 7000.0 + (c.seed % 100) as f64).sin()
        + noise(x / 4300.0, z / 4300.0, c.seed ^ 71) * 600.0;
    let shoulder = (-(across / 3300.0).powi(2)).exp();
    let crest = (-(across / 1050.0).powi(2)).exp();
    let massifs = 0.55 + 0.45 * (noise(along / 1900.0, 0.5, c.seed ^ 17) * 0.5 + 0.5);
    // Nested ridge networks break the broad uplift into connected spurs and valleys.
    // Their amplitudes shrink with wavelength; small independent needles never
    // supply the mountain's full height. Drainage is solved after this relief.
    let mut ridges = 0.0;
    let mut weight = 1.0;
    let mut total = 0.0;
    for (i, scale) in [1800.0, 750.0, 300.0, 120.0].into_iter().enumerate() {
        let ridge = 1.0 - noise(x / scale, z / scale, c.seed ^ (171 + i as u64 * 47)).abs();
        ridges += weight * ridge * ridge;
        total += weight;
        weight *= 0.43;
    }
    let folded = ridges / total;
    let hills = noise(x / 1900.0, z / 1900.0, c.seed ^ 33) * 85.0;
    c.sea_level - 75.0
        + coast
            * (145.0
                + hills
                + shoulder * (220.0 + c.relief * massifs * (0.35 + 0.65 * crest) * folded))
}

fn smooth(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn hash(x: i64, z: i64, seed: u64) -> f64 {
    let mut h = seed
        ^ (x as u64).wrapping_mul(0x9e3779b97f4a7c15)
        ^ (z as u64).wrapping_mul(0xd1b54a32d192ed03);
    h = (h ^ (h >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94d049bb133111eb);
    ((h ^ (h >> 31)) >> 11) as f64 / ((1_u64 << 53) as f64) * 2.0 - 1.0
}

fn noise(x: f64, z: f64, seed: u64) -> f64 {
    let ix = x.floor() as i64;
    let iz = z.floor() as i64;
    let u = smooth(x - x.floor());
    let v = smooth(z - z.floor());
    let a = hash(ix, iz, seed) * (1.0 - u) + hash(ix + 1, iz, seed) * u;
    let b = hash(ix, iz + 1, seed) * (1.0 - u) + hash(ix + 1, iz + 1, seed) * u;
    a * (1.0 - v) + b * v
}
