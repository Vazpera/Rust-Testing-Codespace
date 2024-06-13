mod vec3;

fn main() {
    let u = vec3::new(1.0, 1.0, 1.0);
    let v = vec3::new(1.0, 1.0, 1.0);
    let w = u + v;
    println!("{:?}\n{:?}\n{:?}", u, v, w);
}
