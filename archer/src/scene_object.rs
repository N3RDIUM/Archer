use crate::geometries::base::Geometry;

pub struct SceneObject {
    pub geometry: Box<dyn Geometry + Send + Sync>,
}
