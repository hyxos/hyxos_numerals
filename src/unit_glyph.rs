use crate::Numeral;
use std::f32::consts::TAU;

pub const INDICES: [u32; 66] = [
    0, 1, 4, 1, 4, 5, 1, 2, 5, 2, 5, 6, 2, 3, 6, 3, 6, 7, 4, 5, 8, 5, 8, 9, 6, 7, 10, 7, 10, 11, 8,
    9, 12, 9, 12, 13, 9, 10, 13, 10, 13, 14, 10, 11, 14, 11, 14, 15, 12, 13, 17, 13, 16, 17, 14,
    15, 16, 15, 16, 18, 16, 17, 18, 17, 18, 19,
];

pub fn triangle_lookup(i: usize) -> [u32; 3] {
    let ti: usize = i * 3;
    let mut tr: [u32; 3] = [0; 3];
    for j in 0..3 {
        tr[j] = double_indices()[ti + j]
    }
    tr
}

pub fn double_indices() -> [u32; 132] {
    let mut di: [u32; 132] = [0; 132];
    for i in 0..INDICES.len() {
        di[i] = INDICES[i];
        di[i + 66] = INDICES[i] + 20;
    }
    di
}

pub fn vertices(radians: f32, weight: f32) -> [[f32; 3]; 20] {
    let mut v = [[0.0; 3]; 20];
    let base = radians.cos();
    let height = radians.sin();
    let ratio = radians.tan().abs();
    let wx = weight * -base.signum();
    let wy = weight * -height.signum();
    let mut b = [0.0; 4];
    let mut h = [0.0; 4];
    b[0] = wx / 2.0;
    b[1] = -(base + (wx / 2.0));
    b[2] = wx / ratio;
    b[3] = wx / height.abs();
    h[0] = wy;
    h[1] = -(height + 3.0 * wy);
    h[2] = wy / 2.0 * ratio;
    h[3] = wy / base.abs();
    for i in [1, 5, 9, 13, 16, 18] {
        v[i][0] = base + b[1];
    }
    v[2][0] = base + b[3];
    v[3][0] = base;
    v[6][0] = base + b[3] + b[2];
    v[7][0] = base + b[2];
    v[10][0] = base + b[3] + 2.0 * b[2];
    v[11][0] = base + 2.0 * b[2];
    v[14][0] = base + b[3] + 3.0 * b[2];
    v[15][0] = base + 3.0 * b[2];
    for i in 4..8 {
        v[i][1] = height + h[1] + 2.0 * h[0]
    }
    for i in 8..12 {
        v[i][1] = height + h[1] + h[0]
    }
    for i in 12..16 {
        v[i][1] = height + h[1]
    }
    v[16][1] = height + h[2] + h[3];
    v[17][1] = height + h[3];
    v[18][1] = height + h[2];
    v[19][1] = height;
    v
}

pub fn quadrants(radians: f32) -> [f32; 4] {
    let quad_delta = (TAU / 4.0 - radians) * 2.0;
    let quad2 = radians + quad_delta;
    let quad3 = radians + TAU / 2.0;
    let quad4 = quad3 + quad_delta;
    [radians, quad2, quad3, quad4]
}

pub fn quad_vertices(quadrant: usize, radians: f32, weight: f32) -> [[f32; 3]; 20] {
    let t = quadrants(radians)[quadrant];
    vertices(t, weight)
}

pub fn glyph_vertices(radians: f32, weight: f32, flip: bool) -> [[f32; 3]; 40] {
    let quads = quadrants(radians);
    let mut sides = [quads[0], quads[1]];
    if flip {
        sides[0] = quads[2];
        sides[1] = quads[3];
    }
    let a = vertices(sides[0], weight);
    let b = vertices(sides[1], weight);
    let mut c: [[f32; 3]; 40] = [[0.0; 3]; 40];
    for i in 0..a.len() {
        c[i] = a[i];
        c[i + 20] = b[i];
    }
    c
}

pub fn glyph_vertices2d(radians: f32, weight: f32, flip: bool) -> [[f32; 2]; 40] {
    glyph_vertices(radians, weight, flip).map(|v| [v[0], v[1]])
}

pub fn section_indices(i: usize) -> [u32; 6] {
    let si = i * 6;
    let mut sec: [u32; 6] = [0; 6];
    for j in 0..6 {
        sec[j] = INDICES[si + j]
    }
    sec
}

pub fn double_section_indices(i: usize) -> [u32; 6] {
    let si = i * 6;
    let mut sec: [u32; 6] = [0; 6];
    for j in 0..6 {
        sec[j] = double_indices()[si + j]
    }
    sec
}

pub fn section_keys(u: Numeral) -> Vec<usize> {
    let mut sections: Vec<usize> = vec![10];
    let mut stem: Vec<usize> = vec![0, 3, 5, 8];
    let mut leg: Vec<usize> = vec![2, 4, 7, 9];
    let mut base: Vec<usize> = vec![0, 1, 2];
    let mut bar: Vec<usize> = vec![5, 6, 7];
    match u.row_index() {
        0 => {
            sections.append(&mut base);
            sections.append(&mut bar);
        }
        1 => sections.append(&mut base),
        2 => sections.append(&mut bar),
        3 => (),
        _ => panic!("Can only index vals 0 - 3"),
    }
    match u.col_index() {
        0 => sections.append(&mut leg),
        1 => {
            sections.append(&mut stem);
            sections.append(&mut leg)
        }
        2 => sections.append(&mut stem),
        _ => panic!("Can only index vals 0 - 2"),
    }
    sections.sort_unstable();
    sections.dedup();
    sections
}

pub fn double_section_keys(u: Numeral) -> Vec<usize> {
    let mut sk = section_keys(u);
    let mut dsk: Vec<usize> = section_keys(u).into_iter().map(|k| k + 11).collect();
    dsk.append(&mut sk);
    dsk
}

pub fn section_select(u: Numeral) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::<u32>::new();
    let gs = section_keys(u);
    for i in gs {
        let mut p = section_indices(i).to_vec();
        v.append(&mut p)
    }
    v
}

pub fn double_section_select(u: Numeral) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::<u32>::new();
    let gs = double_section_keys(u);
    for i in gs {
        let mut p = double_section_indices(i).to_vec();
        v.append(&mut p)
    }
    v
}

pub fn y_center(v: [f32; 3], radians: f32, flip: bool) -> [f32; 3] {
    let st = radians.sin();
    let mut sn = -0.5 * st;
    if flip {
        sn = 0.5 * st
    }
    [v[0], v[1] + sn, v[2]]
}

pub fn y_center2d(v: [f32; 2], radians: f32, flip: bool) -> [f32; 2] {
    let st = radians.sin();
    let mut sn = -0.5 * st;
    if flip {
        sn = 0.5 * st
    }
    [v[0], v[1] + sn]
}
