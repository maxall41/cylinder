# Cylinder


Cylinder is an embedded rust library for type-safe inter MCU/MPU communication 
that supports `#![no_std]`. I created it because handling 
complex inter-MCU communication without type-safety
is extremely difficult, but with Cylinder you don't have
to worry about defining unique `u8` values for 
commands and conflicts and other issues with non type-safe communication.


---
## Table Of Contents:
1. [Overview](#Overview)
2. [Installation](#Installation)
3. [Contribution](#Contribution)
---
## Overview:
Cylinder allows for 
type-safe communication by providing a procedural macro that runs at build time that 
you can apply to enum types. e.g:
```rust
use cylinder::CylinerBuildU8;

#[derive(CylinerBuildU8)]
#[derive(Eq, Hash, PartialEq,Debug)]
enum Test {
    MyEnum1,
    MyEnum2,
    MyEnum3,
    MyEnum4
}
```
At build time this enum get's converted to a hash map where each enum type
gets mapped to a unique `u8` value that can be sent over I2C, SPI, CAN or any other protocol.
You can use that hashmap by doing the below:
```rust
let hash : HashMap<&Test, u8> = Test::describe();
```
And then you can use it like this:
```rust
let val : u8 = hash.get(&Test::MyEnum1);
```
Note that to allow the usage of `#![no_std]` instead of using the `std:hashmap` implementation Cylinder uses
hashbrown which is a hasmap library that supports `#![no_std]` so you will need to import `hashbrown` into your code to use the hashmap.
Note that hashbrown when used in a `#![no_std]` enviroment requires that you configure a 
global allocator with the `alloc` package for example with the `embedded-alloc` package.


---
## Installation:
You can install it with:
```shell
cargo add embedded-cylinder
```
## Contribution:
If you have any ideas to improve the library create a PR or issue and i will be happy to review it!