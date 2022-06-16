// This is a simplified version of the Jandom library,
// located at: https://github.com/kallekankaanpaa/jandom
// It's the most accurate version to the original java
// source code that I could find, But it doesn't compile.
// So I'm just going to use what I need.

use std::num::Wrapping;

pub const MULTIPLIER: Wrapping<i64> = Wrapping(0x5DEECE66D);
pub const ADDEND: Wrapping<i64> = Wrapping(0xB);
pub const MASK: Wrapping<i64> = Wrapping((1 << 48) - 1);

const F32_DIV: f32 = (1u32 << 24) as f32;
const F64_DIV: f64 = (1u64 << 53) as f64;

#[derive(Debug, Copy, Clone)]
pub struct Random {
    state: Wrapping<i64>,
}

impl Random {
    /// Creates a new random number generator using a single [u64] seed.
    /// 
    /// This has the same effect as calling the constructor with seed param in Java.
    pub fn new(seed: i64) -> Self {
        Random {
            state: Wrapping((seed) ^ MULTIPLIER.0) & MASK,
        }
    }

    /// Sets the seed to `seed`. This is equivalent to `Random::new`
    pub fn set_seed(&mut self, seed: i64) {
        *self = Random::new(seed);
    }

    /// Steps the RNG, returning up to 48 bits.
    /// 
    /// # Panics
    /// If the amount of requested bits is over 48, this function panics. Use next_i64/next_u64 instead, or multiple calls.
    pub fn next(&mut self, bits: u8) -> i32 {
        if bits > 48 {
            panic!("Too many bits!")
        }

        self.state = (self.state * MULTIPLIER + ADDEND) & MASK;

        (self.state.0 as u64 >> (48 - bits)) as i32
    }

    /// Fills the byte array with random bytes.
    pub fn next_bytes(&mut self, bytes: &mut [u8]) {
        for chunk in bytes.chunks_mut(4) {
            let mut block = self.next_u32();

            for item in chunk {
                *item = (block & 0xFF) as u8;
                block >>= 8;
            }
        }
    }

    /// Returns a uniformly distributed signed 32-bit integer.
    pub fn next_i32(&mut self) -> i32 {
        self.next(32) as i32
    }

    /// Returns a uniformly distributed unsigned 32-bit integer.
    pub fn next_u32(&mut self) -> u32 {
        self.next(32) as u32
    }

    /// Returns a positive random number in the range [0, max), up to 2^31.
    /// The range of the return value is represented by the value `0 <= value < max`.
    /// A maximum of less than 1 is invalid because then no value would satisfy the range.
    /// 
    /// # Panics
    /// If `max` is less than 1, this function panics.
    pub fn next_i32_bound(&mut self, max: i32) -> i32 {
        if max <= 0 {
            panic!("Maximum must > 0")
        }

        if (max as u32).is_power_of_two() {
            let max = max as i64;
            return ((max.wrapping_mul(self.next(31) as i64)) >> 31) as i32;
        }

        let mut bits = self.next(31);
        let mut val = bits % max;

        while bits.wrapping_sub(val).wrapping_add(max - 1) < 0 {
            bits = self.next(31);
            val = bits % max;
        }

        val
    }

    /// Returns a positive random number in the range [0, max), up to 2^31.
    /// The range of the return value is represented by the value `0 <= value < max`.
    /// A maximum of 0 is invalid because then no value would satisfy the range.
    /// Maximums of 2^31 or greater are not supported in Java.
    /// 
    /// # Panics
    /// If `max` reinterpreted as a signed 32-bit integer is less than 1, this function panics.
    pub fn next_u32_bound(&mut self, max: u32) -> u32 {
        self.next_i32_bound(max as i32) as u32
    }

    /// Returns a uniformly distributed signed 64-bit integer.
    pub fn next_i64(&mut self) -> i64 {
        ((self.next(32) as i64) << 32).wrapping_add(self.next(32) as i64)
    }

    /// Returns a uniformly distributed unsigned 64-bit integer.
    pub fn next_u64(&mut self) -> u64 {
        self.next_i64() as u64
    }

    /// Returns a boolean value that has an equal chance of being true or false.
    pub fn next_bool(&mut self) -> bool {
        self.next(1) == 1
    }

    /// Returns a f32 uniformly distributed between 0.0 and 1.0.
    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / F32_DIV
    }

    /// Returns a f64 uniformly distributed between 0.0 and 1.0.
    pub fn next_f64(&mut self) -> f64 {
        let high = (self.next(26) as i64) << 27;
        let low = self.next(27) as i64;

        (high.wrapping_add(low) as f64) / F64_DIV
    }

    pub fn get_seed(&self) -> i64 {
        self.state.0
    }
}

// This is testing stuff
// Rust file generated by Java
const RAND_NEXTBYTES_SEED: i64 = 0;
const RAND_NEXTBYTES: [u8; 128] = [
	0x60, 0xB4, 0x20, 0xBB, 0x38, 0x51, 0xD9, 0xD4,
	0x7A, 0xCB, 0x93, 0x3D, 0xBE, 0x70, 0x39, 0x9B,
	0xF6, 0xC9, 0x2D, 0xA3, 0x3A, 0xF0, 0x1D, 0x4F,
	0xB7, 0x70, 0xE9, 0x8C, 0x03, 0x25, 0xF4, 0x1D,
	0x3E, 0xBA, 0xF8, 0x98, 0x6D, 0xA7, 0x12, 0xC8,
	0x2B, 0xCD, 0x4D, 0x55, 0x4B, 0xF0, 0xB5, 0x40,
	0x23, 0xC2, 0x9B, 0x62, 0x4D, 0xE9, 0xEF, 0x9C,
	0x2F, 0x93, 0x1E, 0xFC, 0x58, 0x0F, 0x9A, 0xFB,
	0x08, 0x1B, 0x12, 0xE1, 0x07, 0xB1, 0xE8, 0x05,
	0xF2, 0xB4, 0xF5, 0xF0, 0xF1, 0xD0, 0x0C, 0x2D,
	0x0F, 0x62, 0x63, 0x46, 0x70, 0x92, 0x1C, 0x50,
	0x58, 0x67, 0xFF, 0x20, 0xF6, 0xA8, 0x33, 0x5E,
	0x98, 0xAF, 0x87, 0x25, 0x38, 0x55, 0x86, 0xB4,
	0x1F, 0xEF, 0xF2, 0x05, 0xB4, 0xE0, 0x5A, 0x00,
	0x08, 0x23, 0xF7, 0x8B, 0x5F, 0x8F, 0x5C, 0x02,
	0x43, 0x9C, 0xE8, 0xF6, 0x7A, 0x78, 0x1D, 0x90,
];
const RAND_NEXT32_SEED: i64 = 123;
const RAND_NEXT32: [u32; 128] = [
	0xB921F1DD, 0x3CBC0495, 0xFDAB8CD1, 0x4D33F0AA, 0x40D7D116, 0x92FA2632, 0x9BDA5745, 0x42460F3A,
	0xCE4D76C3, 0x9F539C8A, 0xE01B0D5F, 0x8C9ED4FC, 0xB74EF490, 0x2986F7DA, 0x1269277B, 0xAE8B47EB,
	0xCBD7C258, 0x4484F122, 0x9426CB04, 0x3E1AD100, 0xE87AEBB4, 0x9A2ECA2B, 0x261F4429, 0x3774C5D4,
	0xF9A802D5, 0x754C29CB, 0x10CAE6D0, 0x16CA6888, 0x11CBEC93, 0x1F4BF868, 0x173B8ACB, 0x568E81E8,
	0x6CA9D9C5, 0xEE9800BA, 0xDFD81749, 0xAD89F9CE, 0x933A01D3, 0x007B58D4, 0x6580D18E, 0x7FF711C4,
	0x4220F288, 0xFB17D764, 0x271D62E6, 0xCD4325ED, 0x42AB892C, 0xC8E71939, 0x7630C9D6, 0x03B11824,
	0x7068E8D1, 0x3C07DB05, 0x361EAA95, 0x7A14C965, 0x9D3D01ED, 0x2460AB64, 0x0890B2BA, 0x7082456B,
	0xB05B7F29, 0x1CAD391C, 0xAF220436, 0xD951849B, 0x13442B77, 0x281BC1A6, 0xF261DB83, 0x6681E820,
	0x6FD7256A, 0xF07463ED, 0xB19DE51D, 0x6A96D456, 0xAD2873C1, 0xD41C5B54, 0xA0F7F631, 0xA9B90D6B,
	0x931FCF44, 0x07E366FE, 0x63521C94, 0xC67EBA43, 0x306AB5D3, 0x6AA9E148, 0x06D739F6, 0x2F76319C,
	0x08141ABB, 0x9CD24B9A, 0xD4959E56, 0x65ADF573, 0xBD76484B, 0xD359525F, 0xF10455B8, 0xD33680A5,
	0xD370C6A7, 0x529B8D51, 0x83A31D16, 0xC940CAD8, 0x98155BDB, 0x9F807FF9, 0x3A703293, 0x5A46A69B,
	0x4D9C7932, 0x2B369E76, 0xB1EE74F6, 0xBBD85DCB, 0x6DA2A9C8, 0x6E245A7C, 0xF7298655, 0x6F7D3038,
	0xB9B5D65E, 0x3E6E7C9F, 0x55CCE610, 0x48CA9889, 0xF552726D, 0x5DB44DD1, 0x7C3E3601, 0x4AB07B5C,
	0x3EDDE67D, 0x34A65529, 0xB1FA1EF1, 0x7536A4B1, 0x382CCBB6, 0xF224ADE6, 0x1C76884D, 0xC9CA3788,
	0x90F8CCB7, 0x82DE39B2, 0x83C41718, 0xBCFFF3C7, 0x27933FA6, 0x563A332A, 0x8D542823, 0xDB9A7661,
];

const RAND_NEXT64_SEED: i64 = 246;
const RAND_NEXT64: [u64; 128] = [
	0xBC2EC2F174B9BA7A, 0x7105C369C188DAB7, 0x675B3ED6DBC9DF76, 0x90C83A7E3100BDC5, 0x915F0D0009C13A6D, 0x613F705DDAC80A80, 0xC60228F178DAE48D, 0x0DC22D3BB4876431,
	0xC02820BB2E5E2B17, 0x2207E420DD6975B2, 0xA71F07A0AA6B372D, 0x1284A032F71A0F67, 0x5D186EFD2B49DEF0, 0xCB40023A4CC4B06D, 0x5CBE8962A0ABF3CA, 0xE2E0125F9FD5EF14,
	0x8E55DC05F095A207, 0x2BE8EA386686748B, 0xE8FE2259FFB3B5AA, 0x0EBE7B01756D7FD9, 0x98AB47FAA310D87E, 0xDC05F4298EC42414, 0xD7999CCBE6981D6A, 0x64D88E18B6E1B5CC,
	0x308EA7632BE46D09, 0xE511BD26B90D1BC7, 0x986A39A3A4C1A77A, 0x381F10EF3145F6F3, 0xBFA1ABAABDBD8F74, 0x7A5843C82263D598, 0x517960EEAC73D29C, 0x744A3E8F7B18E5CC,
	0x1D6EFBA29421C31A, 0x52C814A88348DB2C, 0x9188E25F76C39669, 0x5A413C4684FBFBC8, 0x9065FBFF6AC63D6E, 0x160C86EBDC41885F, 0xC632C5CA9A7629CC, 0x614B9E2B1ED3F3CB,
	0xCA5727D84C049476, 0xF70308B522849DBE, 0xDB02ABAD4C821985, 0xA63EFB93279C04AE, 0xF9F2F9285EC6BD4A, 0x05DD7BAF5DB6A30D, 0xF12ABDA1DE2CAEA7, 0x861A9397DD75EBBD,
	0xF90D61514D845A9A, 0xFC83A1853AE019C3, 0xB1B62EED91FDA51A, 0x7DC3019A2AC0C73A, 0x15A7D196B92B5B26, 0x48059869B4077F89, 0x565B4CF64E03321A, 0x2CD001BBDC40C0D8,
	0xF203D3A0EC0DE843, 0xDA3167922F1920BF, 0x274F1FC347205AB6, 0x659E456663A18840, 0x7E4031FBAA2FA45D, 0xDE9D9BB66206BAF7, 0xD6BC9A85BE5F9A52, 0x891757C328DD9D8F,
	0x1C42B0978D8C396F, 0xC6BAF3945D48EF78, 0xD4C15C0E4E83D925, 0x08E1924174546BD6, 0x94F05548D631378C, 0x40BF1C6D592C85BC, 0x7111FF531551B2BC, 0xCEEA2112AA993399,
	0xA3F64046F745435A, 0x957A7E8645957DF2, 0x61A7FBEF394B0C75, 0x757B17B1EA44C551, 0x85A514AE73569492, 0x996B19A6A721F37D, 0x101B169FE9D6FC05, 0xAF4F95581B540BEA,
	0x20A0F0FECB8AC482, 0xD59E719EFFD6CF72, 0xE18861C7FD0FFDF3, 0xFB57F98326DD6745, 0x1B67F79E5E89EC8B, 0xCD348ABCDCC04B1E, 0x8728CDECBBB27C1A, 0xC42028785D26D6B7,
	0x6BF16751153F14A2, 0x4C95F658A530427F, 0x47F94A3940E5A42B, 0x5709DFBE0766F389, 0xB5E743C6A4C7F1D3, 0x7FAEEF49738456C6, 0x1E1674FA10D88E28, 0xB3CD1A9EC1BABB76,
	0x1A3C8E0EFE2DF4B9, 0x24C1866B929DE0DB, 0x74ADDC232E6DB2EA, 0x10F086A9811A2B30, 0xF1220D1D3DC2A808, 0x91A0DF22B675B3D8, 0x4DA6CDCA056EB29D, 0x162C08341855A8DA,
	0x459BA6480D395F04, 0x69677BD1A975AF8D, 0x9864B8A7F4026B3F, 0x39E74ED1A6663E8F, 0xD03845CEF4D83407, 0xD0FA9A63F38222FB, 0x8D451C9C1B5F5F24, 0x634679DF979AA4DA,
	0x9CAE574E895A56FF, 0xA8ECA0C4F2DCFED9, 0xF3BB0B2864F46B74, 0xEE76CCFCCA6B1D3C, 0x7C5ECE4DBD5DABED, 0xD3909964FA4CD813, 0x532A37F25681CEAC, 0x752D748CF7FE1CAA,
	0xE600BEB3DE75B967, 0xD650BFBB4B31BA45, 0x30E3994564DC7F19, 0x498A5A35B7A7C60C, 0xE8F7854CEF3DE716, 0x359A1CC02D71CA44, 0xA7E5988DF255D160, 0x65D1096353EE34BF,
];
const RAND_NEXT32_BOUND_65536_SEED: i64 = 369;
const RAND_NEXT32_BOUND_65536: [u32; 128] = [
	0x0000BF24, 0x0000C034, 0x000091FD, 0x00006A2F, 0x0000D5DA, 0x00000D3C, 0x0000A6C4, 0x00004A98,
	0x0000CF89, 0x00002BB0, 0x00009370, 0x0000C280, 0x0000251B, 0x0000F7D7, 0x0000DB0C, 0x00000379,
	0x0000FA1D, 0x0000A564, 0x0000ABA4, 0x0000BB4B, 0x0000E2EA, 0x0000221B, 0x0000DEC8, 0x0000E11A,
	0x00005787, 0x0000890A, 0x0000748D, 0x0000F689, 0x0000EE94, 0x000040CD, 0x0000E7E9, 0x0000FE02,
	0x0000E8BD, 0x00001AEF, 0x00009855, 0x000006B3, 0x00001D61, 0x00006E95, 0x00002C28, 0x0000071D,
	0x0000C056, 0x0000574F, 0x0000D68E, 0x0000DCBC, 0x00007586, 0x00008081, 0x00007C71, 0x00003A56,
	0x0000A5A1, 0x0000E091, 0x0000CA6D, 0x00006B8A, 0x0000658A, 0x00008B23, 0x0000FC61, 0x000072FA,
	0x000094B3, 0x0000016B, 0x0000C255, 0x0000C5A7, 0x000023F2, 0x0000B953, 0x0000CFA4, 0x0000E1D5,
	0x0000F9C6, 0x0000B1EC, 0x00000632, 0x00001E1F, 0x0000522B, 0x0000F3CD, 0x00003850, 0x00003701,
	0x000029AA, 0x000090F1, 0x00005F4F, 0x000081B9, 0x00009511, 0x0000BF32, 0x00002A97, 0x0000F3BF,
	0x0000641D, 0x0000CD86, 0x000091AA, 0x0000D1FA, 0x0000148A, 0x00002430, 0x0000CB70, 0x000052F8,
	0x0000A043, 0x00000E30, 0x0000BBFE, 0x000087B1, 0x0000BB0F, 0x00006BCD, 0x0000A8FF, 0x0000B72A,
	0x00007E9D, 0x00007E2F, 0x00004101, 0x000006ED, 0x00009D1F, 0x0000F821, 0x0000BCF1, 0x0000D4D6,
	0x0000FF32, 0x00004B29, 0x0000F183, 0x0000DA99, 0x000027F6, 0x000087EF, 0x0000720E, 0x0000E0C6,
	0x000005CA, 0x000007CE, 0x0000AC81, 0x00000230, 0x000095FC, 0x0000C1F9, 0x0000F7A4, 0x0000E1BF,
	0x00004972, 0x00009A6D, 0x00008F46, 0x00009839, 0x0000AED3, 0x0000FAF7, 0x000026AA, 0x0000E55E,
];
const RAND_NEXT32_BOUND_999999999_SEED: i64 = 369;
const RAND_NEXT32_BOUND_999999999: [u32; 128] = [
	0x23F74229, 0x247F8570, 0x0D63B8EC, 0x3517F409, 0x2F529259, 0x069E5DCA, 0x17C74738, 0x254C77DD,
	0x2C29C6CF, 0x15D82E1D, 0x0E1D503E, 0x25A56591, 0x128DE1D7, 0x31EBB28A, 0x01BCFF53, 0x1717A06F,
	0x1A37860D, 0x220B25C6, 0x35DA72AB, 0x110D9E68, 0x33C947EA, 0x34F284FE, 0x2BC3B329, 0x08EA3CD7,
	0x3A4682DC, 0x2066B070, 0x3859F848, 0x38C3CC91, 0x0D77DCD5, 0x109005DC, 0x03599497, 0x0EB0F2C1,
	0x374AD7BC, 0x16143DDF, 0x038EFB16, 0x24903953, 0x2BA7AD43, 0x2FACB421, 0x32C36E9B, 0x3AC326D6,
	0x04A5E964, 0x029E0976, 0x1D2B141B, 0x17362E4C, 0x34AE2E86, 0x299BC7EF, 0x35C53C94, 0x32C5445A,
	0x09F6FF2E, 0x397D1303, 0x0EBF0E2D, 0x00B59ED5, 0x258FBDF1, 0x2739088E, 0x11F923E6, 0x210F1FD6,
	0x2C37A9E3, 0x35500EC0, 0x1D5B7E7E, 0x0319387C, 0x0F0FEFF0, 0x2915DACD, 0x1C283E34, 0x1B809DC2,
	0x14D50638, 0x0CDDCC1D, 0x2FA7E2D2, 0x0541CDC8, 0x0EEE0E5C, 0x23FE8CDD, 0x154BF7FC, 0x320E804B,
	0x2B284EE0, 0x0D3A965D, 0x2D626301, 0x0A456B16, 0x121831AB, 0x2A1D4B71, 0x297C5A6F, 0x1486BD9A,
	0x071812C8, 0x2264B5EE, 0x083E02AB, 0x21ECE400, 0x35E6F4B3, 0x18E4E2D2, 0x1FFA76C9, 0x03B3FAD7,
	0x037D34EC, 0x20808904, 0x0376B038, 0x12F513D0, 0x22DE23AB, 0x2ED03E47, 0x2594E1AB, 0x31B1CDBD,
	0x13FB163C, 0x085CF2B1, 0x39074010, 0x34C8AAA6, 0x02E525F6, 0x03E76101, 0x1AA6328B, 0x01183C2D,
	0x0F63AD34, 0x2561F7A6, 0x3544E4A5, 0x24B91906, 0x119BDAB6, 0x0C085489, 0x1081FFA2, 0x1BCEE81E,
	0x13552807, 0x37145844, 0x03D33ADE, 0x10A934A3, 0x0E95B7A6, 0x02A29794, 0x03F5FF1C, 0x2827362A,
	0x0ED7F1B4, 0x2027E505, 0x1FCFF777, 0x196B5AF6, 0x023CC975, 0x2C8DA51D, 0x2D6BB04D, 0x08BDDC24,
];

const RAND_NEXTBOOL_SEED: i64 = 492;
const RAND_NEXTBOOL: [bool; 128] = [
	true, true, false, true, false, true, true, true,
	false, false, true, true, true, true, false, false,
	false, true, false, false, true, false, true, true,
	false, false, true, true, false, true, false, true,
	true, false, true, true, false, true, false, true,
	true, false, true, false, false, false, true, false,
	false, true, false, false, true, false, true, false,
	true, true, false, true, false, true, false, true,
	false, false, false, false, false, false, false, true,
	false, false, true, false, true, true, true, false,
	true, false, false, false, false, true, false, false,
	true, false, true, true, true, false, true, true,
	true, false, false, true, false, false, false, false,
	true, false, true, false, false, true, true, false,
	true, true, false, true, true, false, true, true,
	false, false, true, true, true, false, false, true,
];

const RAND_NEXTF32_SEED: i64 = 615;
// f32 bit representation
const RAND_NEXTF32: [u32; 128] = [
	0x3F2D1DA4, 0x3E572B3C, 0x3F55089A, 0x3D99E008, 0x3DB69008, 0x3F1E74FB, 0x3F0606C0, 0x3E468138,
	0x3F4BD621, 0x3F069A1D, 0x3EF2E17E, 0x3E036F00, 0x3F5BB556, 0x3F0CE5B3, 0x3F012184, 0x3C95BB40,
	0x3EDE9866, 0x3F02C529, 0x3ECA5640, 0x3E877166, 0x3F739BD0, 0x3F0A55E4, 0x3F34CD84, 0x3F642915,
	0x3E77A4EC, 0x3E9BA0C4, 0x3E928D52, 0x3EAE9974, 0x3EB074D2, 0x3F5C4927, 0x3EEBBF2E, 0x3CF4E480,
	0x3EE90666, 0x3F15E88E, 0x3EDDBA0C, 0x3F7B379B, 0x3EFDD474, 0x3E111AAC, 0x3F58317D, 0x3EE35290,
	0x3E8B6D94, 0x3E8551A2, 0x3F483A30, 0x3F2E508F, 0x3F5CF600, 0x3EB364FC, 0x3ED35E1E, 0x3F1666F8,
	0x3CBEDB20, 0x3F72F3AE, 0x3D581B80, 0x3F172969, 0x3D4A1F40, 0x3EADB5BA, 0x3E03B8E4, 0x3ED72508,
	0x3F67AB1C, 0x3EA66260, 0x3F08BBEC, 0x3B254300, 0x3F71E7F2, 0x3CB573A0, 0x3E5F6F0C, 0x3EDFB4AA,
	0x3EB7F08A, 0x3EDB0812, 0x3D074CD0, 0x3C612F40, 0x3EC643E0, 0x3F14B99C, 0x3EE48DD2, 0x3F0F28B1,
	0x3ECC16A8, 0x3F75C7DC, 0x3ED6AD94, 0x3EA0139E, 0x3ECE3980, 0x3F419848, 0x3F3F55CD, 0x3F26E413,
	0x3EA0049E, 0x3E6DA9FC, 0x3EB4D6B2, 0x3F0D152C, 0x3D74D2C0, 0x3E46ACC0, 0x3E70B2AC, 0x3F53B218,
	0x3E6730D4, 0x3F5B725C, 0x3D975AB8, 0x3E98BE5A, 0x3EA442B6, 0x3CDCB5A0, 0x3EBAA3C8, 0x3F207EF0,
	0x3F6B9A58, 0x3F0543DF, 0x3F13C93A, 0x3E16B960, 0x3D6A8860, 0x3EB45524, 0x3ED731B8, 0x3F24CB6F,
	0x3E3AF3C8, 0x3E13DFBC, 0x3DF2F8B0, 0x3E14B5A8, 0x3F100AFE, 0x3D13DF60, 0x3F109DA1, 0x3DF41CF8,
	0x3F31051B, 0x3F0E557B, 0x3F3CEA6A, 0x3EB685FA, 0x3EF9190C, 0x3EA4F606, 0x3ECC36D2, 0x3F19DFEC,
	0x3E001808, 0x3EA78034, 0x3ED97F96, 0x3CD19680, 0x3DC897B0, 0x3D4BFDC0, 0x3EB54FB0, 0x3F4812DA,
];

const RAND_NEXTF64_SEED: i64 = 738;
// f64 bit representation
const RAND_NEXTF64: [u64; 128] = [
	0x3FE6054EC36E442F, 0x3FD218B458790EBE, 0x3FCEAAB7DCE89694, 0x3FDEBD290205AFCC, 0x3FE1DCF6EF883DD8, 0x3FEF52A4437827B1, 0x3FED4D1156E1CD01, 0x3FDF1EA280AA9F6A,
	0x3FD8E72476C9E63A, 0x3FEE61873F183AC4, 0x3FE647FD8CD4928F, 0x3FE4265C1D1E72F8, 0x3FE42B35001E70B9, 0x3F8DDE231A8E05C0, 0x3FE465A0D2ED4916, 0x3FD06107B50EE91C,
	0x3FE2C5E6ACBF3181, 0x3FE75DBB2DA1A0AE, 0x3FEA95CB591BF843, 0x3FE02DE4D338FDB8, 0x3FE38823FF550E91, 0x3FDF48B056FD18D8, 0x3FDC7904F7763824, 0x3FD615B4F499795A,
	0x3FE8C392FF168207, 0x3FE78E9956B10DE2, 0x3F9F3CB0D73BD980, 0x3FD41F25F2C5635A, 0x3FEEDE292FA20C32, 0x3FD4FC8B149B7948, 0x3FC80E94314075D0, 0x3FE73884C4238A98,
	0x3FA320368898B470, 0x3FE533DF98E1B620, 0x3FD1E097A3760D7E, 0x3FC5C817ED4D73F0, 0x3FD8D460658AAB2E, 0x3FBE1134D9732770, 0x3FEF9C9A078B2487, 0x3FB9CA3225907580,
	0x3FB2455C3AA73208, 0x3FDF3630F49EBD4A, 0x3FC66CC7F55A7EF0, 0x3FEE2CEA2940BCE1, 0x3FD8139A0E79D8CC, 0x3FE2A4B6C706A830, 0x3FE566D7AA2C8EDE, 0x3FE51F88A91D71AE,
	0x3FE2E1680D3C8CDE, 0x3FEBCBCCED25B0A3, 0x3FD4AF02D7E03DD6, 0x3FEE464AF3007834, 0x3FE155DDC4FDA66D, 0x3FB097C8641A9468, 0x3FEE227B1FCC66D7, 0x3FD04BDB5B013E4E,
	0x3FD90AC2345BD0E8, 0x3FECA43668A92BCB, 0x3FDAEBB69A776AFE, 0x3FE5E864C99DB9E6, 0x3FAA9ACEBD88C270, 0x3FE8F329FD5C9BE0, 0x3FE9078A2BA72A1B, 0x3FD59AC1F1556014,
	0x3FE2766C05828041, 0x3FBC3387C30DC620, 0x3FE3381AA86D6735, 0x3FC1BFDCCE29AE04, 0x3FEA36933FF4D736, 0x3FEC863190508A3E, 0x3FE631B57829301B, 0x3FD14754AC99EA3A,
	0x3FE5D4ED7F4B47EC, 0x3FEEB62D3D0EDBB2, 0x3FEB3356425AC0D3, 0x3FD3662D53281F56, 0x3FD48833858D21D2, 0x3FE60995707AF478, 0x3FE9809A7A20B092, 0x3FEAD27F82C0A184,
	0x3FE7023B5D0C8E09, 0x3FB43711E86C3580, 0x3FE23662A1772EAC, 0x3FEEA07EBCC43CA3, 0x3FC6F26F23A742D8, 0x3FD973D56966492E, 0x3FEE0C85567E6A08, 0x3FB4B8B2AC60A9D0,
	0x3FD8BEFDCEA51528, 0x3FB3D3FD1EBC62B0, 0x3FE2D4D42B92F318, 0x3FEC5763FF54B6C3, 0x3FEC2668EF4A9ABF, 0x3FDAAB90072E34BC, 0x3FD09F660719D096, 0x3FE8C23699CFC796,
	0x3FEE891697868D74, 0x3FE7709F9244B234, 0x3FCADA03CD12F3FC, 0x3F9AF8C09BE55FA0, 0x3FD052CE1802BC9E, 0x3F804DDEBB1B7300, 0x3FED241350866AF6, 0x3FC58331170B4720,
	0x3FE22E78276CCCF8, 0x3FE80638C3356356, 0x3FB65C17272EF270, 0x3FD0D682EFE112BE, 0x3FE3C134494C145C, 0x3FE769A8E2FED198, 0x3FEBB58AC0716BED, 0x3FEAACF92467195A,
	0x3FEFA692ADB2D45C, 0x3FE045D06A4F02BE, 0x3F909734A1244FC0, 0x3FEAE661FD27FF90, 0x3FE53BC9230BFE05, 0x3FC4298DC290D6CC, 0x3F911695E578BBC0, 0x3FE667A7369D1AAF,
	0x3F882C1D3DA490C0, 0x3FDB905898E3BBDA, 0x3FEE30CCAC544512, 0x3FE3E2744E099350, 0x3FE1249A39300313, 0x3FE47177EA8E89CA, 0x3FEFB625218C7ABF, 0x3FE4BA9A6ABF5387,
];

#[test]
fn test_nextbytes() {
    let mut random = Random::new(RAND_NEXTBYTES_SEED);

    let mut bytes = [0; 128];
    random.next_bytes(&mut bytes);

    assert_eq!(&bytes as &[u8], &RAND_NEXTBYTES as &[u8]);
}

#[test]
fn test_next32() {
    let mut random = Random::new(RAND_NEXT32_SEED);

    for (index, &elem) in RAND_NEXT32.iter().enumerate() {
        let gen = random.next_u32();

        if gen != elem {
            panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
        }
    }
}

#[test]
fn test_next64() {
    let mut random = Random::new(RAND_NEXT64_SEED);

    for (index, &elem) in RAND_NEXT64.iter().enumerate() {
        let gen = random.next_u64();

        if gen != elem {
            panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
        }
    }
}

#[test]
fn test_next32_bound_65536() {
    let mut random = Random::new(RAND_NEXT32_BOUND_65536_SEED);

    for (index, &elem) in RAND_NEXT32_BOUND_65536.iter().enumerate() {
        let gen = random.next_u32_bound(65536);

        if gen != elem {
            panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
        }
    }
}

#[test]
fn test_next32_bound_999999999() {
    let mut random = Random::new(RAND_NEXT32_BOUND_999999999_SEED);

    for (index, &elem) in RAND_NEXT32_BOUND_999999999.iter().enumerate() {
        let gen = random.next_u32_bound(999999999);

        if gen != elem {
            panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
        }
    }
}

#[test]
fn test_nextbool() {
    let mut random = Random::new(RAND_NEXTBOOL_SEED);

    for (index, &elem) in RAND_NEXTBOOL.iter().enumerate() {
        let gen = random.next_bool();

        if gen != elem {
            panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
        }
    }
}

#[test]
fn test_nextf32() {
    let mut random = Random::new(RAND_NEXTF32_SEED);

    for (index, &elem) in RAND_NEXTF32.iter().enumerate() {
        let gen = random.next_f32().to_bits();

        if gen != elem {
            panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
        }
    }
}

#[test]
fn test_nextf64() {
    let mut random = Random::new(RAND_NEXTF64_SEED);

    for (index, &elem) in RAND_NEXTF64.iter().enumerate() {
        let gen = random.next_f64().to_bits();

        if gen != elem {
            panic!("mismatch at index {}: expected {}, got {}", index, elem, gen);
        }
    }
}