use std::sync::{Arc, Weak, RwLock};
use std::collections::HashMap;
use crate::node::*;
use std::marker;
use std::f64::consts::PI;

pub struct Godswoods<N, T> where N: GodsnodeProto, T: GodswoodProto<N> {
    woods: Arc<RwLock<HashMap<String, Arc<RwLock<Godswood<N, T>>>>>>,
    store: Arc<Store<N>>,
}

pub trait GodswoodProto<N> where N: GodsnodeProto {
    fn init_nodes(&mut self);
    fn get_nodes_by_depths(&self) -> &Godsnodes<N>;
    fn get_depth(&self) -> usize;
    fn get_root(&self) -> Weak<RwLock<N>>;
}

pub struct Godswood<N, T> where N: GodsnodeProto, T: GodswoodProto<N> {
    wood: T,
    scales: HashMap<usize, f64>,
    base_scale: f64,
    base_gap: f64,
    ph: marker::PhantomData<N>,
}

pub struct Godspoint {

}

impl<N: GodsnodeProto, T: GodswoodProto<N>> Godswood<N, T> {
    pub fn new(wood: T) -> Godswood<N, T> {
        Godswood {
            wood,
            scales: HashMap::new(),
            base_scale: 1.0,
            base_gap: 10.0,
            ph: marker::PhantomData,
        }
    }

    fn calculate_scales(&mut self) {
        let nodes = self.wood.get_nodes_by_depths();
        let depth = self.wood.get_depth();
        let nodes = nodes.read().unwrap();
        for i in 1..depth {
            let items = nodes.get(&i).unwrap();
            let mut kids_max = 0;
            for item in items.iter() {
                if let Some(node) = item.upgrade() {
                    let node = node.read().unwrap();
                    let kids = node.get_children();
                    if kids.len() > kids_max {
                        kids_max = kids.len();
                    }
                } else {
                    error!("Unexpected weak node lost connection");
                }
            }

            let scale: f64;

            if kids_max <= 1 {
                scale = 1.0;
            } else {
                let angle = PI / kids_max as f64;
                scale = 2.0 * 0.5 / angle.sin();
            }

            self.scales.insert(i, scale);
        }
    }

    pub fn render_test(&self) {
        let nodes = self.wood.get_nodes_by_depths();
        let max_depth = self.wood.get_depth();
        let nodes = nodes.read().unwrap();
        for i in 1..max_depth + 1 {
            let items = nodes.get(&i).unwrap();
            info!("Drawing nodes with depth: {}", i);
            let mut kids_max = 0;
            for item in items.iter() {
            }
        }
    }

    pub fn draw_node(node: Weak<RwLock<N>>) {}

    pub fn draw_root(node: Weak<RwLock<N>>) {}

    
    pub fn render(self) {
        let root = self.wood.get_root();
        Godswood::<N, T>::draw_node(root.clone());
        let node  = root.upgrade().unwrap();
        let parent = node.read().unwrap();
        let children = parent.get_children();
        if children.len() > 0 {
        }
    }

}

