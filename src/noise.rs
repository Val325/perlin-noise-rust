//original code on C https://gist.github.com/nowl/828013

//yours seed
const seed: i64 = 88843510;
const hash: [i64; 256] = [208,34,231,213,32,248,233,56,161,78,24,140,71,48,140,254,245,255,247,247,40,
                     185,248,251,245,28,124,204,204,76,36,1,107,28,234,163,202,224,245,128,167,204,
                     9,92,217,54,239,174,173,102,193,189,190,121,100,108,167,44,43,77,180,204,8,81,
                     70,223,11,38,24,254,210,210,177,32,81,195,243,125,8,169,112,32,97,53,195,13,
                     203,9,47,104,125,117,114,124,165,203,181,235,193,206,70,180,174,0,167,181,41,
                     164,30,116,127,198,245,146,87,224,149,206,57,4,192,210,65,210,129,240,178,105,
                     228,108,245,148,140,40,35,195,38,58,65,207,215,253,65,85,208,76,62,3,237,55,89,
                     232,50,217,64,244,157,199,121,252,90,17,212,203,149,152,140,187,234,177,73,174,
                     193,100,192,143,97,53,145,135,19,103,13,90,135,151,199,91,239,247,33,39,145,
                     101,120,99,3,186,86,99,41,237,203,111,79,220,135,158,42,30,154,120,67,87,167,
                     135,176,183,191,253,115,184,21,233,58,129,233,142,39,128,211,118,137,139,255,
                     114,20,218,113,154,27,127,246,250,1,8,198,250,209,92,222,173,21,88,102,219];

pub fn noise2(x: i64, y: i64) -> i64{
    let tmp: i64 = hash[((y + seed) % 256) as usize];
    return  hash[((tmp + x) % 256) as usize];
}

pub fn lin_inter(x: f64, y: f64, s: f64) -> f64{
    return x + s * (y -x);
}

pub fn smooth_inter(x: f64, y: f64, s: f64) -> f64{
    return lin_inter(x, y, s * s * (3.0-2.0*s));
}

pub fn noise2d(x: f64, y: f64) -> f64{
    let x_int: i64 = x as i64;
    let y_int: i64 = y as i64;
    
    let x_frac: f64 = x - (x_int as f64 );
    let y_frac: f64 = y - (y_int as f64 );

    let s: i64 = noise2(x_int, y_int);
    let t: i64 = noise2(x_int+1, y_int);
    let u: i64 = noise2(x_int, y_int+1);
    let v: i64 = noise2(x_int+1, y_int+1);
    let low: f64 = smooth_inter(s as f64, t as f64, x_frac);
    let high: f64 = smooth_inter(u as f64, v as f64, x_frac);
    return smooth_inter(low, high, y_frac);
}

pub fn perlin2d(x: f64, y: f64, freq: f64, depth: i64) -> f64{
    let mut xa: f64 = x*freq;
    let mut ya: f64 = y*freq;
    let mut amp: f64 = 1.0;
    let mut fin: f64 = 0.0;
    let mut div: f64 = 0.0;

    let mut i: i64 = 0;
    while i < depth { 
        div = div + (256.0 * amp);
        fin = fin + (noise2d(xa, ya) * amp);
        amp = amp / 2.0;
        xa = xa * 2.0;
        ya = ya * 2.0;
        i += 1;
    }

    return fin/div;
}
