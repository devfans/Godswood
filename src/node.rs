use std::sync::{Weak, RwLock};

pub type Godsnode = RwLock<GodsnodeType>;
pub trait GodsnodeProto {
}

pub enum GodsnodeType {
    Root,
    Node,
    Leaft,
}

pub enum ServiceType {
    General,
}


pub struct NodeProto {
    id: u64,
    pub name: String,
    pub display_name: String,
    pub node_type: GodsnodeType,
    parents: Vec<Weak<Node>>,
    children: Vec<Weak<Node>>,
    pub service_type: ServiceType,
}

impl GodsnodeProto for NodeProto {
}
