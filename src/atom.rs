use crate::vector::Vector3;

pub struct Atom {
    pub position: Vector3,
    pub velocity: Vector3,
    pub force: Vector3,
    pub mass: f32
}
