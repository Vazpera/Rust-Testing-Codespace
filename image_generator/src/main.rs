use cgmath::{prelude::*, Vector3};
use chrono;
use image::RgbImage;
use rand::{self, rngs::StdRng, Rng, SeedableRng};
const WHITE_SOLID: Material = Material {
    diffuse_color: Vector3::new(1.0, 1.0, 1.0),
    specular_color: Vector3::new(1.0, 1.0, 1.0),
    emission_color: Vector3::new(0.0, 0.0, 0.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 0.1,
};
const RED_SOLID: Material = Material {
    diffuse_color: Vector3::new(1.0, 0.1, 0.1),
    specular_color: Vector3::new(1.0, 0.0, 0.0),
    emission_color: Vector3::new(0.0, 0.0, 0.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 0.1,
};
const GREEN_SOLID: Material = Material {
    diffuse_color: Vector3::new(0.1, 1.0, 0.1),
    specular_color: Vector3::new(1.0, 0.0, 0.0),
    emission_color: Vector3::new(0.0, 0.0, 0.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 0.1,
};
const WHITE_MIRROR: Material = Material {
    diffuse_color: Vector3::new(1.0, 1.0, 1.0),
    specular_color: Vector3::new(1.0, 1.0, 1.0),
    emission_color: Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 1.0,
};
const RED_MIRROR: Material = Material {
    diffuse_color: Vector3::new(1.0, 0.0, 0.0),
    specular_color: Vector3::new(0.9, 0.8, 0.8),
    emission_color: Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.0,
    specular_chance: 1.0,
    smoothness: 1.0,
};
const CYAN_MIRROR: Material = Material {
    diffuse_color: Vector3::new(1.0, 1.0, 1.0),
    specular_color: Vector3::new(0.8, 0.9, 0.9),
    emission_color: Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.0,
    specular_chance: 0.9,
    smoothness: 1.0,
};
const WHITE_LIGHT: Material = Material {
    diffuse_color: Vector3::new(0.0, 0.0, 0.0),
    specular_color: Vector3::new(0.0, 0.0, 0.0),
    emission_color: Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 1.0,
    specular_chance: 0.0,
    smoothness: 0.0,
};
const CYAN_LIGHT: Material = Material {
    diffuse_color: Vector3::new(0.0, 0.0, 0.0),
    specular_color: Vector3::new(0.0, 0.0, 0.0),
    emission_color: Vector3::new(0.0, 1.0, 1.0),
    emission_strength: 1.0,
    specular_chance: 0.0,
    smoothness: 0.0,
};
const WHITE_GLOW: Material = Material {
    diffuse_color: Vector3::new(1.0, 1.0, 1.0),
    specular_color: Vector3::new(1.0, 1.0, 1.0),
    emission_color: Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.2,
    specular_chance: 0.1,
    smoothness: 0.1,
};

#[derive(Clone, Copy)]
struct Material {
    diffuse_color: Vector3<f32>,
    specular_color: Vector3<f32>,
    emission_color: Vector3<f32>,
    emission_strength: f32,
    smoothness: f32,
    specular_chance: f32,
}

#[derive(Clone, Copy)]
struct Sphere {
    radius: f32,
    position: Vector3<f32>,
    material: Material,
}
#[derive(Clone, Copy)]
struct Triangle {
    a: Vector3<f32>,
    b: Vector3<f32>,
    c: Vector3<f32>,
    material: Material,
}

#[derive(Clone, Copy)]
struct Ray {
    origin: Vector3<f32>,
    dir: Vector3<f32>,
}

#[derive(Copy, Clone)]
struct HitInfo {
    position: Vector3<f32>,
    normal: Vector3<f32>,
    material: Material,
}

trait Intersectable {
    fn intersect(&self, _: Ray) -> Option<HitInfo> {
        return None;
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Option<HitInfo> {
        let l = self.position - ray.origin;
        let t_ca = l.dot(ray.dir);
        let d_squared = l.dot(l);
        if t_ca < 0.0 {
            return None;
        }
        let d = (d_squared - t_ca * t_ca).sqrt();
        if (self.radius - d) < 0.0 {
            return None;
        }
        let t_hc = (self.radius * self.radius - d * d).sqrt();
        let t_0 = t_ca - t_hc;
        let p_0 = ray.origin + ray.dir * t_0;
        let normal = (p_0 - self.position).normalize();
        Some(HitInfo {
            position: p_0,
            normal,
            material: self.material,
        })
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: Ray) -> Option<HitInfo> {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let ray_cross_e2 = ray.dir.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None; // This ray is parallel to this triangle.
        }
        let inv_det = 1.0 / det;
        let s = ray.origin - self.a;
        let u = inv_det * s.dot(ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let s_cross_e1 = s.cross(e1);
        let v = inv_det * ray.dir.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = inv_det * e2.dot(s_cross_e1);

        if t > f32::EPSILON {
            let intersection_point = ray.origin + ray.dir * t;
            let normal = e1.cross(e2).normalize();
            return Some(HitInfo {
                position: intersection_point,
                normal: normal,
                material: self.material,
            });
        } else {
            // This means that there is a line intersection but not a ray intersection.
            return None;
        }
    }
}

fn random_normal_dist(rng: &mut StdRng) -> f32 {
    let theta: f32 = 2.0 * std::f32::consts::PI * rng.gen::<f32>();
    let rho: f32 = (-2.0 * rng.gen::<f32>().ln()).sqrt();
    return rho * theta.cos();
}

fn random_direction(rng: &mut StdRng) -> Vector3<f32> {
    let vec = Vector3 {
        x: random_normal_dist(rng),
        y: random_normal_dist(rng),
        z: random_normal_dist(rng),
    }
    .normalize();
    return vec;
}
fn reflect(incoming: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    return (incoming) - normal * (2.0 * normal.dot(incoming));
}

fn trace(
    bounces: u8,
    objects: &Vec<&dyn Intersectable>,
    start: Ray,
    rng: &mut StdRng,
) -> Vector3<f32> {
    let mut ray = start;
    let mut ray_color: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
    let mut incoming_light: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

    for _ in 0..bounces {
        let intr = shoot(objects, ray);
        match intr {
            None => break,
            Some(hit) => {
                let mat = hit.material;

                let diffuse_dir = (hit.normal.normalize() + random_direction(rng)).normalize();
                let reflect_dir = reflect(ray.dir, hit.normal);
                let is_specular: f32 = match mat.specular_chance > rng.gen::<f32>() {
                    true => 1.0,
                    false => 0.0,
                };
                let ray_dir = (diffuse_dir * (1.0 - mat.smoothness * is_specular)
                    + reflect_dir * (mat.smoothness * is_specular))
                    .normalize();
                let emitted_light = mat.emission_color * mat.emission_strength;

                incoming_light += emitted_light.mul_element_wise(ray_color);
                ray_color = (mat.diffuse_color * (1.0 - mat.smoothness * is_specular)
                    + mat.specular_color * (mat.smoothness * is_specular))
                    .mul_element_wise(ray_color);
                ray.origin = hit.position;
                ray.dir = ray_dir;
            }
        }
    }
    return incoming_light;
}

const WIDTH: u32 = 250;
const HEIGHT: u32 = 250;
const FOCAL_LENGTH: f32 = 1.0;
const BOUNCES: u8 = 10;
const RAYS: u16 = 160;

fn shoot(objects: &Vec<&dyn Intersectable>, ray: Ray) -> Option<HitInfo> {
    let mut dist = f32::INFINITY;
    let mut hit: Option<HitInfo> = None;
    for object in objects {
        let new_hit = object.intersect(ray);
        match new_hit {
            None => (),
            Some(j) => {
                if (j.position - ray.origin).magnitude() < dist {
                    hit = Some(j);
                    dist = (j.position - ray.origin).magnitude();
                }
            }
        }
    }

    return hit;
}

fn main() {
    let now = chrono::Local::now().timestamp_millis();

    let mut rng = rand::rngs::StdRng::from_entropy();
    let sphere_1 = Sphere {
        radius: 4.0,
        position: Vector3::new(-4.0, -6.0, 0.0),
        material: WHITE_MIRROR,
    };
    let sphere_2 = Sphere {
        radius: 2.0,
        position: Vector3::new(2.0, -8.0, 0.0),
        material: WHITE_SOLID,
    };

    let sphere_3 = Sphere {
        radius: 1.0,
        position: Vector3::new(4.0, -9.0, 9.0),
        material: WHITE_LIGHT,
    };

    let floor_1 = Triangle {
        a: Vector3::new(10.0, -10.0, -10.0),
        b: Vector3::new(-10.0, -10.0, 10.0),
        c: Vector3::new(10.0, -10.0, 10.0),
        material: WHITE_SOLID,
    };
    let floor_2 = Triangle {
        a: Vector3::new(10.0, -10.0, -10.0),
        b: Vector3::new(-10.0, -10.0, -10.0),
        c: Vector3::new(-10.0, -10.0, 10.0),
        material: WHITE_SOLID,
    };
    let roof_1 = Triangle {
        a: Vector3::new(10.0, 10.0, 10.0),
        b: Vector3::new(-10.0, 10.0, -10.0),
        c: Vector3::new(10.0, 10.0, -10.0),
        material: WHITE_SOLID,
    };
    let roof_2 = Triangle {
        a: Vector3::new(-10.0, 10.0, -10.0),
        b: Vector3::new(10.0, 10.0, 10.0),
        c: Vector3::new(-10.0, 10.0, 10.0),
        material: WHITE_SOLID,
    };
    let light_1 = Triangle {
        a: Vector3::new(5.0, 9.9, 5.0),
        b: Vector3::new(-5.0, 9.9, -5.0),
        c: Vector3::new(5.0, 9.9, -5.0),
        material: WHITE_LIGHT,
    };
    let light_2 = Triangle {
        a: Vector3::new(-5.0, 9.9, -5.0),
        b: Vector3::new(5.0, 9.9, 5.0),
        c: Vector3::new(-5.0, 9.9, 5.0),
        material: WHITE_LIGHT,
    };
    let wall_s_1 = Triangle {
        a: Vector3::new(-10.0, 10.0, -10.0),
        b: Vector3::new(10.0, -10.0, -10.0),
        c: Vector3::new(10.0, 10.0, -10.0),
        material: WHITE_MIRROR,
    };
    let wall_s_2 = Triangle {
        a: Vector3::new(-10.0, -10.0, -10.0),
        b: Vector3::new(10.0, -10.0, -10.0),
        c: Vector3::new(-10.0, 10.0, -10.0),
        material: RED_SOLID,
    };
    let wall_e_1: Triangle = Triangle {
        a: Vector3::new(10.0, 10.0, -10.0),
        b: Vector3::new(10.0, -10.0, 10.0),
        c: Vector3::new(10.0, 10.0, 10.0),
        material: RED_SOLID,
    };
    let wall_e_2: Triangle = Triangle {
        a: Vector3::new(10.0, 10.0, -10.0),
        b: Vector3::new(10.0, -10.0, -10.0),
        c: Vector3::new(10.0, -10.0, 10.0),
        material: RED_SOLID,
    };
    let wall_w_1: Triangle = Triangle {
        a: Vector3::new(-10.0, 10.0, -10.0),
        b: Vector3::new(-10.0, -10.0, 10.0),
        c: Vector3::new(-10.0, -10.0, -10.0),
        material: GREEN_SOLID,
    };
    let wall_w_2: Triangle = Triangle {
        a: Vector3::new(-10.0, 10.0, -10.0),
        b: Vector3::new(-10.0, 10.0, 10.0),
        c: Vector3::new(-10.0, -10.0, 10.0),
        material: GREEN_SOLID,
    };
    let wall_n_1 = Triangle {
        a: Vector3::new(-10.0, 10.0, 10.0),
        b: Vector3::new(10.0, -10.0, 10.0),
        c: Vector3::new(-10.0, -10.0, 10.0),
        material: WHITE_SOLID,
    };
    let wall_n_2 = Triangle {
        a: Vector3::new(10.0, 10.0, 10.0),
        b: Vector3::new(10.0, -10.0, 10.0),
        c: Vector3::new(-10.0, 10.0, 10.0),
        material: WHITE_SOLID,
    };

    let objects: Vec<&dyn Intersectable> = vec![
        &roof_1, &roof_2, &sphere_1, &sphere_2, &sphere_3, &floor_1, &floor_2, &wall_s_1,
        &wall_s_2, &wall_e_1, &wall_e_2, &wall_w_1, &wall_w_2, &wall_n_1, &wall_n_2, &light_1,
        &light_2,
    ];

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let u: f32 = (i as f32 / WIDTH as f32) * 2.0 - 1.0;
            let v: f32 = -((j as f32 / HEIGHT as f32) * 2.0 - 1.0);
            let vec: Vector3<f32> = Vector3::new(u, v, FOCAL_LENGTH).normalize();

            let view_ray = Ray {
                dir: vec,
                origin: Vector3::new(0.0, 0.0, -10.0),
            };

            let mut color: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..RAYS {
                color += trace(BOUNCES, &objects, view_ray, &mut rng) * 1.0 / (RAYS as f32);
            }
            image.get_pixel_mut(i, j).0 = [
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8,
            ];
        }
        image.save("output.png").expect("Image successfully saved");
        println!("{i}");
    }
    let then = chrono::Local::now().timestamp_millis();
    println!("{}", then - now);
}
