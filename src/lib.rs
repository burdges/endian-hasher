// Copyright 2016 Jeffrey Burdges.

/*!
These simple wrapper types for `Hasher` change the endianness
used when hashing primitive numeric types.

# Examples

```
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use endian_hasher::*;

assert_eq!( {
    let mut h1 = HasherToLE(DefaultHasher::new());
    h1.write_i16(-3);
    h1.finish()
}, {
    let mut h0 = DefaultHasher::new();
    h0.write_i16( i16::to_le(-3) );
    h0.finish()
} );

assert_eq!( {
    let mut h1 = HasherToBE(DefaultHasher::new());
    h1.write_u32(79);
    h1.finish()
}, {
    let mut h0 = DefaultHasher::new();
    h0.write_u32( 79u32.to_be() );
    h0.finish()
} );

assert_eq!( {
    let mut h1 = HasherSwapBytes(DefaultHasher::new());
    h1.write_u64(0x12345678);
    h1.finish()
}, {
    let mut h0 = DefaultHasher::new();
    h0.write_u64( u64::swap_bytes(0x12345678) );
    h0.finish()
} );
```
*/

use std::hash::{Hasher}; // Hash

#[doc(hidden)]
macro_rules! conv_hasher{
    ($i:ident,$t:ident) =>{

// #[doc="Apply a primitive numeric type's `stringify!($t)` method before hashing."]
// pub struct $i<H: Hasher>(pub H);

impl<H: Hasher> Hasher for $i<H> {
    fn finish(&self) -> u64 { self.0.finish() }
    fn write(&mut self, bytes: &[u8]) { self.0.write(bytes) }

    fn write_u8(&mut self, n: u8) { self.0.write_u8( u8::$t(n) ) }
    fn write_u16(&mut self, n: u16) { self.0.write_u16( u16::$t(n) ) }
    fn write_u32(&mut self, n: u32) { self.0.write_u32( u32::$t(n) ) }
    fn write_u64(&mut self, n: u64) { self.0.write_u64( u64::$t(n) ) }
    fn write_usize(&mut self, n: usize) { self.0.write_usize( usize::$t(n)) }
    fn write_i8(&mut self, n: i8) { self.0.write_i8( i8::$t(n) ) }
    fn write_i16(&mut self, n: i16) { self.0.write_i16( i16::$t(n) ) }
    fn write_i32(&mut self, n: i32) { self.0.write_i32( i32::$t(n) ) }
    fn write_i64(&mut self, n: i64) { self.0.write_i64( i64::$t(n) ) }
    fn write_isize(&mut self, n: isize) { self.0.write_isize( isize::$t(n) ) }
}

    }
} // macro_rules! conv_hasher

/// Apply a primitive numeric type's `to_be` method before hashing.
pub struct HasherToBE<H: Hasher>(pub H);
conv_hasher!(HasherToBE, to_be );

/// Apply a primitive numeric type's `to_le` method before hashing.
pub struct HasherToLE<H: Hasher>(pub H);
conv_hasher!(HasherToLE, to_le );

/// Apply a primitive numeric type's `from_be` method before hashing.
pub struct HasherFromBE<H: Hasher>(pub H);
conv_hasher!(HasherFromBE, from_be );

/// Apply a primitive numeric type's `from_le` method before hashing.
pub struct HasherFromLE<H: Hasher>(pub H);
conv_hasher!(HasherFromLE, from_le );

/// Apply a primitive numeric type's `swap_bytes` method before hashing.
pub struct HasherSwapBytes<H: Hasher>(pub H);
conv_hasher!(HasherSwapBytes, swap_bytes );


/* 
#[cfg(test)]
mod tests {
#![allow(deprecated)]

use super::*;
use std::hash::{SipHasher24, Hasher};

#[test]
fn it_works() {

}

} // mod tests
*/

