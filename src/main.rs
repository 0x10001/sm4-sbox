fn main() {
    verify_sbox();
}

const fn linear(a: u8) -> u8 {
    let mut m = 0xd3u8;
    let mut out = m;
    let mut i = 8;

    while i > 0 {
        i -= 1;
        let ones = (a & m).count_ones() as u8;
        out = out.rotate_left(1) ^ (ones & 1);
        m = m.rotate_right(1);
    }

    out
}

const fn gen_inv() -> [u8; 256] {
    let g = 0x1f5usize;
    let mut inv = [0; 256];
    inv[1] = 1;

    let mut t = 1usize;
    let mut stack = [0u8; 128];

    let mut i = 0;
    while i < 127 {
        t = t << 1;
        if t > 0xff {
            t ^= g;
        }
        stack[i] = t as u8;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        t = t << 1;
        if t > 0xff {
            t ^= g;
        }
        let a = stack[i];
        inv[a as usize] = t as u8;
        inv[t] = a;
    }

    inv
}

fn verify_sbox() {
    let inv_table = gen_inv();

    let inv = move |a: u8| inv_table[a as usize];

    let mut hi = 0u128;
    let mut lo = 0u128;
    let mut i = 0;

    println!("    00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f");
    for row in 0..16 {
        print!("{:02x} ", row << 4);
        for _ in 0..16 {
            let s = linear(inv(linear(i)));
            i = i.wrapping_add(1);
            print!(" {s:02x}");

            if s > 0x7f {
                hi |= 1 << (s - 0x80);
            } else {
                lo |= 1 << s;
            }
        }
        println!();
    }

    assert!(hi == u128::MAX);
    assert!(lo == u128::MAX);
}
