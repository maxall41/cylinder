# Cylinder


Cylinder is an embedded rust library for type-safe inter MCU/MPU communication 
that supports `#![no_std]`. I created it because handling 
complex inter-MCU communication without type-safety
is extremely difficult, but with Cylinder you don't have
to worry about defining unique `u8` values for 
commands, conflicts, and other issues with non type-safe communication.


---
## Table Of Contents:
1. [Overview](#Overview)
2. [Usage Across Binaries](#usage-across-binaries)
2. [Installation](#Installation)
3. [Examples](#Examples)
4. [Contribution](#Contribution)
---
## Overview:
Cylinder allows for 
type-safe communication by providing a procedural macro that runs at build time that 
you can apply to enum types. e.g:
```rust
use cylinder::CylinerBuildU8;

#[derive(CylinerBuildU8)]
enum Test {
    MyEnum1,
    MyEnum2,
    MyEnum3,
    MyEnum4
}
```

At build time an implementation is added for the enum that maps each variant of the enum to
a unique value. Which you can get like this:

```rust
let val : u8 = Test::get_u8(Test::MyEnum2);
```

---

## Usage across binaries:

To use Cylinder across multiple binaries you can define a local library
that contains your type interface and then import that library into your binaries.

---
## Examples:

You can find examples in the `examples/` directory in the git repo

---
## Installation:
You can install it with:
```shell
cargo add embedded_cylinder
```
## Contribution:
If you have any ideas to improve the library create a PR or issue and i will be happy to review it!