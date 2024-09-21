use std::cmp;

const ROUND_CONSTANTS: &[u32] = &[
    0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5, 0x3956C25B, 0x59F111F1, 0x923F82A4, 0xAB1C5ED5,
    0xD807AA98, 0x12835B01, 0x243185BE, 0x550C7DC3, 0x72BE5D74, 0x80DEB1FE, 0x9BDC06A7, 0xC19BF174,
    0xE49B69C1, 0xEFBE4786, 0x0FC19DC6, 0x240CA1CC, 0x2DE92C6F, 0x4A7484AA, 0x5CB0A9DC, 0x76F988DA,
    0x983E5152, 0xA831C66D, 0xB00327C8, 0xBF597FC7, 0xC6E00BF3, 0xD5A79147, 0x06CA6351, 0x14292967,
    0x27B70A85, 0x2E1B2138, 0x4D2C6DFC, 0x53380D13, 0x650A7354, 0x766A0ABB, 0x81C2C92E, 0x92722C85,
    0xA2BFE8A1, 0xA81A664B, 0xC24B8B70, 0xC76C51A3, 0xD192E819, 0xD6990624, 0xF40E3585, 0x106AA070,
    0x19A4C116, 0x1E376C08, 0x2748774C, 0x34B0BCB5, 0x391C0CB3, 0x4ED8AA4A, 0x5B9CCA4F, 0x682E6FF3,
    0x748F82EE, 0x78A5636F, 0x84C87814, 0x8CC70208, 0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2
];

const MAGIC_CONSTANTS: &[u32] = &[
    0x99036946, 0xE99DB8E7, 0xE3AE2FA7, 0xA339740, 0xF06EB6A9, 0x92FF9B65, 0x28F7873, 0x9070E316
];

pub struct Sha256Custom {
    state: [u32; 8],
}

impl Sha256Custom {
    pub fn new(state: [u32; 8]) -> Sha256Custom {
        Sha256Custom {
            state,
        }
    }

    pub fn encrypt(&mut self, data: &mut Vec<u8>) -> String {
        let data_len = data.len();
        let data_len_bits = data_len * 8;
        let no_pad_len_bits = data_len_bits + 1 + 64;
        let pad_len_bits = (512_usize.wrapping_sub(no_pad_len_bits)) % 512;
        let pad_len = (pad_len_bits - 7) / 8;
        
        data.push(b'\x80');
        data.resize(data.len() + pad_len, 0);
        data.extend((data_len_bits as u64).to_be_bytes());
        
        for start in (0..data.len()).step_by(64) {
            let chunk = &data[start..(start + 64)];
            let mut round_state = Vec::new();

            for i in (0..64).step_by(4) {
                round_state.push(u32::from_be_bytes(chunk[i..(i + 4)].try_into().unwrap()));
            }

            round_state.extend_from_within(0..);
            round_state.extend_from_within(0..);

            for i in 16..64 {
                let s0 = Self::rotate_right(round_state[i - 15], 7) ^ Self::rotate_right(round_state[i - 15], 18) ^ (round_state[i - 15] >> 3);
                let s1 = Self::rotate_right(round_state[i - 2], 17) ^ Self::rotate_right(round_state[i - 2], 19) ^ (round_state[i - 2] >> 10);

                round_state[i] = round_state[i - 16].wrapping_add(s0).wrapping_add(round_state[i - 7]).wrapping_add(s1);
            }

            let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = 
            (self.state[0], self.state[1], self.state[2], self.state[3], self.state[4], self.state[5], self.state[6], self.state[7]);

            for i in 0..64 {
                let s0 = Self::rotate_right(a, 2) ^ Self::rotate_right(a, 13) ^ Self::rotate_right(a, 22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let t2 = s0.wrapping_add(maj);

                let s1 = Self::rotate_right(e, 6) ^ Self::rotate_right(e, 11) ^ Self::rotate_right(e, 25);
                let ch = (e & f) ^ ((!e) & g);
                let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(ROUND_CONSTANTS[i]).wrapping_add(round_state[i]);
                
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }

            let ah = [a, b, c, d, e, f, g, h];
            for (i, c) in ah.into_iter().enumerate() {
                self.state[i] = self.state[i].wrapping_add(c);
            }
        }

        let mut encrypted_bytes: Vec<u8> = Vec::new();
        for i in 0..self.state.len() {
            encrypted_bytes.extend(self.state[i].to_be_bytes());
        }

        hex::encode(encrypted_bytes)
    }

    pub fn scramble(enc: &mut String) -> Vec<u8> {
        let unaligned_chunk: String;
        if enc.len() % 8 != 0 {
            let align_len = enc.len() - enc.len() % 8;
            unaligned_chunk = enc[align_len..].to_string();
            enc.truncate(align_len);
            enc.push_str("\x00\x00\x00\x00\x00\x00\x00\x00")
        } else {
            unaligned_chunk = String::default();
        }

        let mut scrambled: Vec<u8> = Vec::new();

        let mut x: u32 = 0x6479B873;
        let mut y: u32 = 0x48853AFC;

        for offset in (0..enc.len()).step_by(8) {
            x ^= u32::from_be_bytes(<[u8; 4]>::try_from(enc[offset..(offset + 4)].as_bytes()).unwrap());
            y ^= u32::from_be_bytes(<[u8; 4]>::try_from(enc[(offset + 4)..(offset + 8)].as_bytes()).unwrap());

            for _ in 0..4 {
                for i in 0..2 {
                    y ^= x;

                    let a = MAGIC_CONSTANTS[4 * i].wrapping_add(y);
                    let b = a.wrapping_sub(1).wrapping_add(Self::rotate_left(a, 1));

                    x ^= b ^ Self::rotate_left(b, 4);

                    let c = MAGIC_CONSTANTS[4 * i + 1].wrapping_add(x);
                    let d = c.wrapping_add(1).wrapping_add(Self::rotate_left(c, 2));

                    let e = MAGIC_CONSTANTS[4 * i + 2].wrapping_add(d ^ Self::rotate_left(d, 8));
                    let f = Self::rotate_left(e, 1).wrapping_sub(e);

                    y ^= (x | f) ^ Self::rotate_left(f, 16);

                    let g = MAGIC_CONSTANTS[4 * i + 3].wrapping_add(y);
                    x ^= g.wrapping_add(1).wrapping_add(Self::rotate_left(g, 2));
                }
            }
            
            scrambled.extend(x.to_be_bytes());
            scrambled.extend(y.to_be_bytes());
        }

        if !unaligned_chunk.is_empty() {
            let last_scramble = scrambled.split_off(scrambled.len() - 8);
            let limit: usize = cmp::min(last_scramble.len(), unaligned_chunk.len());

            for (i, l_s) in last_scramble.into_iter().enumerate().take(limit) {
                scrambled.push(l_s ^ unaligned_chunk.as_bytes()[i]);
            }

        }
        scrambled
    }

    fn rotate_left(n: u32, k: u32) -> u32 {
        (n << k) | (n >> (32 - k))
    }

    fn rotate_right(n: u32, k: u32) -> u32 {
        Self::rotate_left(n, 32 - k)
    }
}