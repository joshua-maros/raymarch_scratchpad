use crate::SdfObject;

pub struct Scene {
    objects: Vec<Box<dyn SdfObject>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add_object<T: SdfObject + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}
