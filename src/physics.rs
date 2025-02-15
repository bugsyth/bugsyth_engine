use physics_object::PhysicsObject;
use shapes::Shape;
use solutions::solve_collision;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tests::test_collision;

pub mod physics_object;
pub mod shapes;
pub mod solutions;
pub mod tests;

pub struct World {
    objects: HashMap<u32, Rc<RefCell<PhysicsObject>>>,
    next_id: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 0,
        }
    }

    /// Clones the Rc<RefCell<PhysicsObject>>
    /// Needs to be removed using remove_object using the id returned from this function
    pub fn add_object(&mut self, object: &Rc<RefCell<PhysicsObject>>) -> u32 {
        self.objects.insert(self.next_id, object.clone());
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn remove_object(&mut self, id: u32) {
        self.objects.remove(&id);
    }

    /// Check and solves collisions
    pub fn update(&mut self) {
        // Dereference key so that self.objects isn't borrowed when getting objects to test collisions
        let keys: Vec<_> = self.objects.keys().map(|key| *key).collect();
        for (i, key1) in keys.iter().enumerate() {
            for key2 in keys.iter().skip(i + 1) {
                let mut obj = self.objects.get(key1).unwrap().borrow_mut();
                let mut other = self.objects.get(key2).unwrap().borrow_mut();
                let colliding = test_collision(&obj.shape, &other.shape);
                if colliding {
                    solve_collision(&mut obj, &mut other);
                }
            }
        }
    }
}
