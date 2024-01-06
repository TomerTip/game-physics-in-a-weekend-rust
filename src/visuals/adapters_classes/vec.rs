use physics::vec::*;

use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::math::Vec4;

#[derive(Debug, Clone, Copy)]
pub struct Vec2Adapter {
    pub vec: vec2d::Vec2d,
    pub bevy_vec: bevy::math::Vec2
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3Adapter {
    pub vec: vec3d::Vec3d,
    pub bevy_vec: bevy::math::Vec3
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4Adapter {
    pub vec: vec4d::Vec4d,
    pub bevy_vec: bevy::math::Vec4
}

impl Vec2Adapter {
    pub fn new(vec: vec2d::Vec2d) -> bevy::math::Vec2 {
        bevy::math::Vec2::new(vec.x, vec.y)
    }
}

struct Vec3Adapter(bevy::math::Vec3);

impl Vec3Adapter {
    fn new(vec: vec3d::Vec3d) -> Self {
        bevy::math::Vec3::new(vec.x, vec.y, vec.z)
    }
}

struct Vec4Adapter(bevy::math::Vec4);

impl Vec4Adapter {
    fn new(vec: vec4d::Vec4d) -> Self {
        bevy::math::Vec4::new(vec.x, vec.y, vec.z, vec.w)
    }
}