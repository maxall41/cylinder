use embedded_cylinder::CylinerBuildU8;




#[derive(CylinerBuildU8)]
union XYZ {
    cubes: u8,
    eggs: u16
}