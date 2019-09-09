use std::sync::{Arc, Weak, RwLock};
use std::collections::HashMap;

pub type NodeStore<T> where T: GodsnodeProto = HashMap<u64, Arc<Godsnode<T>>>;
pub type NodeIndexStore = HashMap<String, u64>;

pub type Store<T> where T: GodsnodeProto = RwLock<StoreProto<T>>;
pub struct StoreProto<T> where T: GodsnodeProto {
    id: u64,
    store: NodeStore<T>,
    index: NodeIndexStore,
}

impl<T: GodsnodeProto> StoreProto<T> {
    pub fn new() -> Arc<Store<T>> {
        Arc::new(RwLock::new(StoreProto {
            id: 0,
            store: HashMap::new(),
            index: HashMap::new(),
        }))
    }
}


pub type Godsnodes<T> where T: GodsnodeProto = Arc<RwLock<HashMap<usize, Vec<Weak<Godsnode<T>>>>>>;
pub type Godsnode<T> where T: GodsnodeProto = RwLock<T>;
pub trait GodsnodeProto {
    fn get_children(&self) -> Vec<Weak<RwLock<Self>>>;
    fn get_parents(&self) -> Vec<Weak<RwLock<Self>>>;
}


pub enum GodsnodeType {
    Root,
    Node,
    Leaft,
}

pub enum ServiceType {
    General,
}


pub type Node = Godsnode<NodeProto>;
pub struct NodeProto {
    id: u64,
    pub name: String,
    pub display_name: String,
    pub node_type: GodsnodeType,
    parents: Vec<Weak<Node>>,
    children: Vec<Weak<Node>>,
    pub service_type: ServiceType,
    pub app_meta_map: AppMetaMap,
}


#[derive(Clone)]
pub struct AppMeta {
    pub path: NodePath,
}

impl AppMeta {
    pub fn new() -> AppMeta {
        AppMeta {
            path: NodePath::new_path(),
        }
    }

    pub fn parse_app_name(path: &String) -> Option<String> {
        if !path.starts_with('.') { return None }
        let tokens: Vec<&str> = path.split('.').collect();
        if tokens.len() > 1 && tokens[1].len() > 0 {
            return Some(tokens[1].to_string());
        }
        None
    }
}

#[derive(Clone)]
pub struct NodePath {
    path: String,
    depth: usize,
}

impl NodePath {
    pub fn new_path() -> NodePath {
        NodePath {
            path: String::new(),
            depth: 0,
        }
    }

    pub fn append(&mut self, name: &String) {
        self.path.push_str(&(".".to_owned() + name));
        self.depth += 1;
    }

    pub fn new(root: String) -> NodePath {
        NodePath {
            path: root,
            depth: 1,
        }
    }

    pub fn read(&self) -> String {
        self.path.clone()
    }

    pub fn read_depth(&self) -> usize {
        self.depth
    }
}

pub type AppMetaMap = HashMap<String, AppMeta>;


