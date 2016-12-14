This crate provides wrappers for `Hasher` that change the endianness
used when hashing primitive numeric types.

### Documentation

There is documentation with examples at [https://docs.rs/endian-hasher/](https://docs.rs/endian-hasher/)


### Installation


This crate works with Cargo and is on
[crates.io](https://crates.io/crates/endian-hasher).  Add it to your `Cargo.toml` with:

```toml
[dependencies]
endianhasher = "^0.1"
```

Use the crate like:

```rust
extern crate endian_hasher;

use std::hash::{Hasher, SipHasher24};

use endianhasher::{HasherToLE, HasherToBE};

let mut h1 = HasherToLE(SipHasher24::new_with_keys(k0,k1));
-3i64.hash(&mut h1);
h1.finish()
```

