use embedded_cylinder::CylinerBuildU8;

#[derive(CylinerBuildU8)]
enum Test {
    XQ,
    YZ,
    FN,
    RX
}



fn main() {
    let val : u8 = Test::get_u8(Test::YZ);
    println!("{}",val);
}
