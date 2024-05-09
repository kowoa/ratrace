use super::hittable::Hittable;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
