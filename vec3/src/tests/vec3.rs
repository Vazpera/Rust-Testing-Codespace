use crate::Vec3;

#[test]
fn test_add() {
    let u = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let v = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let w = Vec3 {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    assert_eq!(u+v, w)
}
