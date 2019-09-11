use crate::node::*;
use crate::tree::*;
use std::sync::{Arc, RwLock};
use amethyst::ecs::{Component, DenseVecStorage };


pub type Castle = Godspoint<NodeProto>;
pub type Forest = Godswoods<NodeProto, TreeProto>;

pub struct Godspoint<N> where N: GodsnodeProto + 'static + Send + Sync {
    pub node: Arc<Godsnode<N>>,
}

impl<N: GodsnodeProto + 'static + Send + Sync> Component for Godspoint<N> {
    type Storage = DenseVecStorage<Self>;
}


impl<N: GodsnodeProto + 'static + Send + Sync> Default for Godspoint<N> {
    fn default() -> Self {
        Self {
            node: Arc::new(RwLock::new(N::new()))
        }
    }
}
