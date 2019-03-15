use crate::asset;

use specs::prelude::*;

pub struct Transform(pub nalgebra::Similarity3<f32>);

impl Component for Transform {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub dist: f32,
    pub focus: nalgebra::Point3<f32>,
    pub proj: nalgebra::Perspective3<f32>,
}

impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}

#[derive(Clone, Copy)]
pub struct Light {
    pub pos: nalgebra::Vector3<f32>,
    pub intensity: f32,
    pub color: [f32; 3],
    pub _pad: f32,
}

impl Component for Light {
    type Storage = DenseVecStorage<Self>;
}

pub struct Mesh(pub asset::MeshHandle);

impl Component for Mesh {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct ActiveCamera;

impl Component for ActiveCamera {
    type Storage = NullStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Helmet;

impl Component for Helmet {
    type Storage = NullStorage<Self>;
}

// pub struct Environment<B: hal::Backend> {
//     mesh: Mesh<B>,
//     hdr: Texture<B>,
//     irradiance: Texture<B>,
//     spec_filtered: Texture<B>,
//     bdrf: Texture<B>,
// }