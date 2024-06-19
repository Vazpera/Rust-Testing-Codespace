
pub const WHITE_SOLID: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    specular_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 0.1,
};
pub const RED_SOLID: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(1.0, 0.1, 0.1),
    specular_color: cgmath::Vector3::new(1.0, 0.0, 0.0),
    emission_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 0.1,
};
pub const GREEN_SOLID: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(0.1, 1.0, 0.1),
    specular_color: cgmath::Vector3::new(1.0, 0.0, 0.0),
    emission_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 0.1,
};
pub const WHITE_MIRROR: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    specular_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.0,
    specular_chance: 0.1,
    smoothness: 1.0,
};
pub const RED_MIRROR: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(1.0, 0.0, 0.0),
    specular_color: cgmath::Vector3::new(0.9, 0.8, 0.8),
    emission_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.0,
    specular_chance: 1.0,
    smoothness: 1.0,
};
pub const CYAN_MIRROR: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    specular_color: cgmath::Vector3::new(0.8, 0.9, 0.9),
    emission_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.0,
    specular_chance: 0.9,
    smoothness: 1.0,
};
pub const WHITE_LIGHT: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
    specular_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
    emission_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 1.0,
    specular_chance: 0.0,
    smoothness: 0.0,
};
pub const CYAN_LIGHT: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
    specular_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
    emission_color: cgmath::Vector3::new(0.0, 1.0, 1.0),
    emission_strength: 1.0,
    specular_chance: 0.0,
    smoothness: 0.0,
};
pub const WHITE_GLOW: crate::Material = crate::Material {
    diffuse_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    specular_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_color: cgmath::Vector3::new(1.0, 1.0, 1.0),
    emission_strength: 0.2,
    specular_chance: 0.1,
    smoothness: 0.1,
};
