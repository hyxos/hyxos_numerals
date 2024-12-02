use std::f32::consts::{PI, TAU};

pub fn diacritic_vertices(diacritic: u8, sample_rate: usize) -> Vec<[f32; 3]> {
    let mut v: Vec<[f32; 3]> = Vec::new();
    let step = 2.0 / sample_rate as f32;
    let mut x = -1.0;
    let tau_step = TAU / sample_rate as f32;
    let mut theta: f32 = 0.0;
    let radius = 1.0 / PI;

    fn ta(r: f32, theta: f32) -> [f32; 2] {
        [
            (r / 2.0).sqrt() * theta.sin(),
            (r / 2.0).sqrt() * theta.cos(),
        ]
    }

    fn shay(r: f32, x: f32) -> [f32; 2] {
        [
            -((x * PI).sin() / (2.0 * PI)) + r,
            -((x * PI).sin() / (2.0 * PI)) - r,
        ]
    }

    fn ree(r: f32, x: f32) -> [f32; 2] {
        [
            (-(x * PI).cos().abs() / PI) + 2.0 * r,
            -(x * PI).cos().abs() / PI,
        ]
    }

    fn jo(r: f32, x: f32) -> [f32; 2] {
        [(x / PI).abs(), (x / PI).abs() - 2.0 * r]
    }

    fn wue(r: f32, x: f32) -> f32 {
        -3.0 / PI * x * x + r
    }

    fn wuxi(r: f32, index: usize, sample_rate: usize) -> f32 {
        let wuxi0 = -(r * PI / 4.0).sqrt();
        let step = (-wuxi0 * 2.0 * index as f32) / sample_rate as f32;
        wuxi0 + step
    }

    fn wuyi(r: f32, x: f32) -> f32 {
        -6.0 / PI * x * x - r / 2.0
    }

    for i in 0..=sample_rate {
        match diacritic {
            0 => {
                let [x, y] = ta(radius, theta);
                if i == sample_rate {
                    v.push([x, y, 0.0]);
                    let stitch = v[1].clone();
                    v.push(stitch);
                } else {
                    v.push([x, y, 0.0])
                }
            }
            1 => {
                let [u, l] = shay(radius, x);
                v.push([x, u, 0.0]);
                v.push([x, l, 0.0]);
            }
            2 => {
                let [u, l] = ree(radius, x);
                v.push([x, u, 0.0]);
                v.push([x, l, 0.0]);
            }
            3 => {
                let [u, l] = jo(radius, x);
                v.push([x, u, 0.0]);
                v.push([x, l, 0.0]);
            }
            4 => {
                let ye = wue(radius, x);
                let xi = wuxi(radius, i, sample_rate);
                let yi = wuyi(radius, xi);
                v.push([x, ye, 0.0]);
                v.push([xi, yi, 0.0]);
            }
            _ => println!("Invalid input!"),
        }
        x += step;
        theta += tau_step;
    }
    v
}

pub fn gen_diacritic_indices(diacritic: u8, sample_rate: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    match diacritic {
        0 => {
            for j in 0..sample_rate {
                let i = j as u32;
                v.push(0);
                v.push(i + 1);
                v.push(i + 2);
            }
        }
        _ => {
            for j in 0..=((sample_rate - 1) * 2) {
                let i = j as u32;
                if i % 2 == 0 {
                    v.push(i);
                    v.push(i + 1);
                    v.push(i + 3);
                    v.push(i);
                    v.push(i + 2);
                    v.push(i + 3);
                }
            }
        }
    }
    v
}
