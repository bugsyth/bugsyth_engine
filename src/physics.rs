use physics_object::PhysicsObject;
use shapes::Shape;
use solutions::solve_collision;
use std::{collections::HashMap, ptr};
use tests::test_collision;

mod physics_object;
mod shapes;
mod solutions;
mod tests;

pub struct World<'a> {
    objects: HashMap<u32, &'a mut PhysicsObject>,
    next_id: u32,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_object(&mut self, object: &'a mut PhysicsObject) -> u32 {
        self.objects.insert(self.next_id, object);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn remove_object(&mut self, id: u32) {
        self.objects.remove(&id);
    }

    pub fn update(&mut self) {
        // Dereference key so that self.objects isn't borrowed when getting objects to test collisions
        let keys: Vec<_> = self.objects.keys().map(|key| *key).collect();
        for (i, key1) in keys.iter().enumerate() {
            for key2 in keys.iter().skip(i + 1) {
                // Can't have 2 mutable references
                // Create pointers then read and write to memory
                let (ptr1, ptr2) = {
                    let a: Option<*mut PhysicsObject> =
                        self.objects.get_mut(key1).map(|x| *x as *mut _);
                    let b: Option<*mut PhysicsObject> =
                        self.objects.get_mut(key2).map(|x| *x as *mut _);
                    (a, b)
                };
                if let (Some(p1), Some(p2)) = (ptr1, ptr2) {
                    unsafe {
                        let (mut obj, mut other) = (ptr::read(p1), ptr::read(p2));
                        let colliding = test_collision(&obj.shape, &other.shape);
                        if colliding {
                            solve_collision(&mut obj, &mut other);
                            ptr::write(p1, obj);
                            ptr::write(p2, other);
                        }
                    }
                }
            }
        }
    }
}
