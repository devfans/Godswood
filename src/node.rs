use std::sync::{Arc, Weak, RwLock};
use std::collections::HashMap;
use serde_json::Value;
use crate::misc::*;

pub type Godsnodes<T> where T: GodsnodeProto = Arc<RwLock<HashMap<usize, Vec<Weak<Godsnode<T>>>>>>;
pub type Godsnode<T> where T: GodsnodeProto = RwLock<T>;
pub type GodsnodeQ<T> = Vec<Weak<Godsnode<T>>>;
pub trait GodsnodeProto {
    fn new() -> Self;
    fn get_children(&self) -> &Vec<Weak<RwLock<Self>>>;
    fn get_parents(&self) -> &Vec<Weak<RwLock<Self>>>;
    fn add_parent(&mut self, node: Weak<Godsnode<Self>>);
    fn add_child(&mut self, node: Weak<Godsnode<Self>>);
}


pub type GodsnodeStore<T> where T: GodsnodeProto= HashMap<u64, Arc<Godsnode<T>>>;
pub type GodsnodeIndexStore = HashMap<String, u64>;

pub struct GodsstoreProto<T> where T: GodsnodeProto {
    id: u64,
    store: GodsnodeStore<T>,
    index: GodsnodeIndexStore,
}

pub type Godsstore<T> where T: GodsnodeProto = RwLock<GodsstoreProto<T>>;
impl<T: GodsnodeProto> GodsstoreProto<T> {
    pub fn new() -> Arc<Godsstore<T>> {
        Arc::new(RwLock::new(GodsstoreProto {
            id: 0,
            store: HashMap::new(),
            index: HashMap::new(),
        }))
    }
}


pub trait GodsstoreOps<T> where T: GodsnodeProto {
    fn new_node(&self) -> Arc<Godsnode<T>>;
    fn add_node(&self, raw: &Value, name: String) -> Arc<Node>;
    fn add_app_node(&self, raw: &Value) -> Arc<Node>;
    fn add_leaf_node(&self, name: &String, raw: &Value) -> Arc<Node>;
    fn update_index(&self, name: &String, index: u64);
    fn get_weak_node(&self, path: &String) -> Option<Weak<Node>>;
}

pub enum GodsnodeType {
    Root,
    Node,
    Leaf,
}

pub enum GodsnodeClass {
    General,
}


pub struct InitNodeQ<T> where T: GodsnodeProto {
    pub app_meta: GodswoodMeta,
    pub nodes: GodsnodeQ<T>,
}

#[derive(Clone)]
pub struct GodswoodMeta {
    pub path: GodsnodePath,
}

impl GodswoodMeta {
    pub fn new() -> GodswoodMeta {
        GodswoodMeta {
            path: GodsnodePath::new_path(),
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
pub struct GodsnodePath {
    path: String,
    depth: usize,
}

impl GodsnodePath {
    pub fn new_path() -> GodsnodePath {
        GodsnodePath {
            path: String::new(),
            depth: 0,
        }
    }

    pub fn append(&mut self, name: &String) {
        self.path.push_str(&(".".to_owned() + name));
        self.depth += 1;
    }

    pub fn new(root: String) -> GodsnodePath {
        GodsnodePath {
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

pub type GodswoodMetaMap = HashMap<String, GodswoodMeta>;


// ---------------- Sample node -------------------------------
pub type Node = Godsnode<NodeProto>;

pub struct NodeProto {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub node_type: GodsnodeType,
    pub parents: Vec<Weak<Node>>,
    pub children: Vec<Weak<Node>>,
    pub service_type: GodsnodeClass,
    pub app_meta_map: GodswoodMetaMap,
}

impl GodsnodeProto for NodeProto {
    fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            display_name: String::new(),
            node_type: GodsnodeType::Node,
            parents: Vec::new(),
            children: Vec::new(),
            service_type: GodsnodeClass::General,
            app_meta_map: HashMap::new(),
        }
    }
    fn get_children(&self) -> &Vec<Weak<RwLock<Self>>> {
        &self.children
    }
    fn get_parents(&self) -> &Vec<Weak<RwLock<Self>>> {
        &self.parents
    }
    fn add_parent(&mut self, node: Weak<Godsnode<Self>>) {
        self.parents.push(node);
    }
    fn add_child(&mut self, node: Weak<Godsnode<Self>>) {
        self.children.push(node);
    }
}

pub type Store = Godsstore<NodeProto>;

impl GodsstoreOps<NodeProto> for Arc<Godsstore<NodeProto>> {
    fn new_node(&self) -> Arc<Godsnode<NodeProto>> {
        let mut node = NodeProto::new();
        let mut store = self.write().unwrap();
        let id = store.id;
        node.id = id;
        store.id += 1;
        let new_node = Arc::new(RwLock::new(node));
        store.store.insert(id, new_node.clone());
        new_node
    }
    fn add_node(&self, raw: &Value, name: String) -> Arc<Node> {
        let node = self.new_node();
        {
            let mut state = node.write().unwrap();
            state.name = name;
            state.display_name = raw.get_str("display_name", "new node");
            state.node_type = GodsnodeType::Node;
        }
        node
    }

    fn add_leaf_node(&self, name: &String, raw: &Value) -> Arc<Node> {
        let node = self.add_node(raw, name.clone());
        {
            let mut state = node.write().unwrap();
            state.node_type = GodsnodeType::Leaf;
        }
        node
    }
    fn add_app_node(&self, raw: &serde_json::Value) -> Arc<Node> {
        let name = raw.get_str("name", "new_application");
        let node = self.add_node(raw, name);
        {
            let mut state = node.write().unwrap();
            state.node_type = GodsnodeType::Root;
        }
        node
    }

    fn update_index(&self, name: &String, index: u64) {
        let mut state = self.write().unwrap();
        state.index.insert(name.clone(), index);
    }

    fn get_weak_node(&self, path: &String) -> Option<Weak<Node>> {
        let state = self.read().unwrap();
        if let Some(id) = state.index.get(path) {
            if let Some(node) = state.store.get(&id) {
                return Some(Arc::downgrade(&node.clone()));
            }
        }
        None
    }
}


