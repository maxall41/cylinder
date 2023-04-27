use embedded_cylinder::CylinerBuildU8;

#[derive(CylinerBuildU8)]
enum Test {
    V1,
    V2,
    V3,
    V4
}

#[derive(CylinerBuildU8)]
enum XYZ {
    X,
    Y,
    Z,
    A,
    B,
    C
}

#[test]
fn test_enum_has_unique_vals_u8() {
    assert_eq!(Test::get_u8(Test::V1), 0);
    assert_eq!(Test::get_u8(Test::V2), 1);
    assert_eq!(Test::get_u8(Test::V3), 2);
    assert_eq!(Test::get_u8(Test::V4), 3);
}

#[test]
fn xyz_enum_has_unique_vals_u8() {
    assert_eq!(XYZ::get_u8(XYZ::X), 0);
    assert_eq!(XYZ::get_u8(XYZ::Y), 1);
    assert_eq!(XYZ::get_u8(XYZ::Z), 2);
    assert_eq!(XYZ::get_u8(XYZ::A), 3);
    assert_eq!(XYZ::get_u8(XYZ::B), 4);
    assert_eq!(XYZ::get_u8(XYZ::C), 5);
}

#[test]
fn failures_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/*.rs");
}
