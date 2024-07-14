use crate::geometries::base::Geometry;
use crate::materials::base::Material;

pub struct SceneObject {
    pub geometry: Box<dyn Geometry + Send + Sync>,
    pub material: Box<dyn Material + Send + Sync>
}
