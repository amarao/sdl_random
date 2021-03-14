use std::collections::VecDeque;
const DEPTH:usize = 1;


pub struct Factor{
    factor: u128,
    byte: u8
}


impl Factor{
    pub fn new(start: u128)-> Self{
        Factor{
            factor: start,
            byte: 0
        }
    }
    #[inline(always)]
    fn step(&mut self){
        // self.factor +=1;
        self.factor ^= self.factor << 17;
        self.factor ^= self.factor >> 5;
        self.factor ^= self.factor << 44;
    }
    #[inline(always)]
    pub fn next(&mut self) -> u8{
        let res = self.factor.to_ne_bytes()[self.byte as usize];
        self.byte = self.byte + 1;
        if self.byte == 16 {
            self.step();
            self.byte = 0;
        };
        res
        
    }
    #[inline(always)]
    pub fn next128(&mut self) -> u128{
        self.step();
        self.factor
    }
}


pub fn noise_fill(bytearray: &mut [u8], factor: &mut Factor) {
    
    let u128_slice: &mut [u128] = bytemuck::cast_slice_mut(bytearray);
       
    for lll in u128_slice{
        *lll = factor.next128();
    }
 }

pub fn noise_flakes(bytearray: &mut [u8], factor: &mut Factor) {
    
    let u32_slice: &mut [u32] = bytemuck::cast_slice_mut(bytearray);
    for lll in u32_slice{
        if factor.next128() % 111 == 0{
            *lll = factor.next128() as u32;
        }else{
            *lll = 0;
        }
    }
}

pub fn fade_in_out(bytearray: &mut [u8], frames: u64){
    let single = frames as u8;
    
    let base = {
        if frames & 0x100 == 0{
            // print!("{} -> {},", frames, single);
            u32::from_ne_bytes([single, single, single, 255])
        }
        else{
            // print!("{} => {},", frames, 0-single-1);
            u32::from_ne_bytes([0-single-1, 0-single-1, 0-single-1, 255])
        }
    };
    let u32_slice: &mut [u32] = bytemuck::cast_slice_mut(bytearray);
    for lll in u32_slice{
        *lll = base;
    }
}