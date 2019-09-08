use std::sync::{Arc, Weak, RwLock};
use std::collections::HashMap;

pub type Godswood = RwLock<GodswoodProto>;
pub trait GodswoodProto {
    fn init_nodes(&mut self);
}

pub struct HeartTree<T> where T: GodswoodProto {
    wood: Arc<RwLock<T>>,
    scales: HashMap<usize, f64>,
    base_scale: f64,
    depth: usize,
}

impl<T: GodswoodProto> HeartTree<T> {
    pub fn new(wood: Arc<RwLock<T>>) -> HeartTree<T> {
        HeartTree {
            wood,
            scales: HashMap::new(),
            base_scale: 1.0,
            depth: 1,
        }
    }
    fn calculate_scales(&mut self) {
    }
}

pub struct TreeProto {
}

impl TreeProto {
}

impl GodswoodProto for TreeProto {
    fn init_nodes(&mut self) {}
}


