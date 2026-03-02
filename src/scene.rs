use std::collections::BTreeSet;
use crate::metrics::{Bounds3, Vector3};
use crate::sparse_array::SparseArray;

#[derive(Debug, PartialEq)]
pub struct Scene {
    ///
    /// The bounds of the scene
    ///
    bounds: Bounds3<i32>,

    ///
    /// Whether the bounds have changed since the last reset
    ///
    bounds_changed: bool,

    ///
    /// The object tree
    ///
    objects: BTreeSet<ObjectRef>,
}

impl Scene {

    ///
    /// Creates a new scene
    ///
    pub fn new(bounds: Bounds3<i32>) -> Self {
        Self {
            bounds,
            bounds_changed: true,
            objects: BTreeSet::new(),
        }
    }

    ///
    /// Sets the bounds of the scene
    ///
    pub fn set_bounds(&mut self, bounds: Bounds3<i32>) {
        self.bounds = bounds;
        self.bounds_changed = true;
    }

    ///
    /// Updates the scene
    ///
    pub fn update<T: Object>(&mut self, objects: &SparseArray<T>) {
        if self.bounds_changed {
            // bounds have changed, so we need to update the entire scene
            self.objects.clear();
        } else {
            // bounds have not changed, so only objects that have moved or have been removed must be checked
            self.objects.retain(|object_ref| {
                !objects.get(object_ref.id).map(Object::has_moved).unwrap_or(true)
            })
        }
        // Check for objects to add to the scene
        for object in objects.iter() {
            let position = object.get_position();
            if (self.bounds_changed || object.has_moved()) && self.bounds.point_is_within(position) {
                self.objects.insert(ObjectRef {
                    id: object.get_id(),
                    x: position.get_x(),
                    y: position.get_y(),
                    z: position.get_z(),
                });
            }
        }
    }

    ///
    /// Resets the scene
    ///
    pub fn reset<T: Object>(&mut self, objects: &mut SparseArray<T>) {
        self.bounds_changed = false;
        objects.iter_mut().for_each(|object| object.reset_moved());
    }

    ///
    /// An iterator over the objects in the scene
    ///
    pub fn iter<'a, T: Object>(&'a self, objects: &'a SparseArray<T>) -> Iter<'a, T> {
        Iter {
            objects,
            ids: self.objects.iter(),
        }
    }
}

pub struct Iter<'a, T> {
    objects: &'a SparseArray<T>,
    ids: std::collections::btree_set::Iter<'a, ObjectRef>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ids.next().and_then(|r| self.objects.get(r.id)) {
            Some(object) => Some(object),
            None => None,
        }
    }
}

pub trait Object {
    fn get_id(&self) -> usize;

    fn get_position(&self) -> &Vector3<i32>;

    fn has_moved(&self) -> bool;

    fn reset_moved(&mut self);
}

#[derive(Debug, PartialEq, Eq)]
struct ObjectRef {
    id: usize,
    x: i32,
    y: i32,
    z: i32,
}

impl PartialOrd for ObjectRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.z.cmp(&other.z) {
            std::cmp::Ordering::Equal => {
                match self.y.cmp(&other.y) {
                    std::cmp::Ordering::Equal => self.x.cmp(&other.x),
                    ordering => ordering,
                }
            },
            ordering => ordering,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestObject {
        id: usize,
        position: Vector3<i32>,
        moved: bool,
    }

    impl TestObject {
        fn new(id: usize, x: i32, y: i32, z: i32, moved: bool) -> TestObject {
            TestObject {
                id,
                position: Vector3::new(x, y, z),
                moved,
            }
        }

        fn set_position(&mut self, x: i32, y: i32, z: i32) {
            self.position = Vector3::new(x, y, z);
            self.moved = true;
        }
    }

    impl Object for TestObject {

        fn get_id(&self) -> usize {
            self.id
        }

        fn get_position(&self) -> &Vector3<i32> {
            &self.position
        }

        fn has_moved(&self) -> bool {
            self.moved
        }

        fn reset_moved(&mut self) {
            self.moved = false;
        }
    }

    #[test]
    fn new_scene() {
        let mut scene = Scene::new(Bounds3::new(0, 0, 0, 100, 0, 100));
        let mut objects = SparseArray::new();
        objects.insert(TestObject::new(0, 0,1,0, false));
        objects.insert(TestObject::new(1, 0,0,1, false));
        objects.insert(TestObject::new(2, 0,0,0, false));
        objects.insert(TestObject::new(3, -1,0,0, false));

        scene.update(&objects);

        assert_eq!(scene.objects.len(), 3);

        let mut ordered = scene.iter(&objects);
        assert_eq!(ordered.next(), objects.get(2));
        assert_eq!(ordered.next(), objects.get(0));
        assert_eq!(ordered.next(), objects.get(1));
        assert_eq!(ordered.next(), None);
    }

    #[test]
    fn scene_update() {
        let mut scene = Scene::new(Bounds3::new(0, 100, 0, 100, 0, 100));
        let mut objects = SparseArray::new();
        objects.insert(TestObject::new(0, 0,1,0, false));
        objects.insert(TestObject::new(1, 0,0,1, false));
        objects.insert(TestObject::new(2, 0,0,0, false));
        objects.insert(TestObject::new(3, -1,0,0, false));

        scene.update(&objects);
        scene.reset(&mut objects);

        objects.get_mut(0).unwrap().set_position(-10, 0, 0);
        objects.get_mut(1).unwrap().set_position(1, 1, 1);
        scene.update(&objects);

        assert_eq!(scene.objects.len(), 2);

        let mut ordered = scene.iter(&objects);
        assert_eq!(ordered.next(), objects.get(2));
        assert_eq!(ordered.next(), objects.get(1));
        assert_eq!(ordered.next(), None);
    }

    #[test]
    fn scene_change_bounds() {
        let mut scene = Scene::new(Bounds3::new(0, 100, 0, 100, 0, 100));
        let mut objects = SparseArray::new();
        objects.insert(TestObject::new(0, 0,50,0, false));
        objects.insert(TestObject::new(1, 50,50,1, false));
        objects.insert(TestObject::new(2, 0,75,1, false));

        scene.update(&objects);
        scene.reset(&mut objects);

        assert_eq!(scene.objects.len(), 3);
        let mut ordered = scene.iter(&objects);
        assert_eq!(ordered.next(), objects.get(0));
        assert_eq!(ordered.next(), objects.get(1));
        assert_eq!(ordered.next(), objects.get(2));
        assert_eq!(ordered.next(), None);


        scene.set_bounds(Bounds3::new(0, 25, 0, 100, 0, 100));
        scene.update(&objects);

        assert_eq!(scene.objects.len(), 2);
        let mut ordered = scene.iter(&objects);
        assert_eq!(ordered.next(), objects.get(0));
        assert_eq!(ordered.next(), objects.get(2));
        assert_eq!(ordered.next(), None);
    }
}