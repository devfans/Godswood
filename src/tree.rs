use std::sync::{Arc, Weak, RwLock};
use std::collections::{ HashMap, VecDeque };
use crate::node::*;
use std::marker;
use std::f64::consts::PI;
use serde_json::Value;
use crate::misc::*;

pub struct Godswoods<N, T> where N: GodsnodeProto, T: GodswoodProto<N> {
    pub woods: Arc<RwLock<HashMap<String, Arc<RwLock<Godswood<N, T>>>>>>,
    pub store: Arc<Godsstore<N>>,
}

impl<N: GodsnodeProto, T: GodswoodProto<N>> Godswoods<N, T> {
    pub fn new() -> Self {
        Self {
            woods: Arc::new(RwLock::new(HashMap::new())),
            store: GodsstoreProto::<N>::new(),
        }
    }

    pub fn add_wood(&mut self, raw: &Value) {
        let mut wood: T = T::default(self.store.clone());
        wood.parse_from_json(raw);
        wood.init_nodes();
        let mut woods = self.woods.write().unwrap();
        let name = wood.read_name();
        let mut godswood = Godswood {
            wood,
            scales: HashMap::new(),
            base_scale: 1.0,
            base_gap: 10.0,
            ph: marker::PhantomData,
        };

        godswood.calculate_scales();
        woods.insert(name, Arc::new(RwLock::new(godswood)));
    }
}

pub trait GodswoodProto<N> where N: GodsnodeProto {
    fn default(store: Arc<Godsstore<N>>) -> Self;
    fn init_nodes(&mut self);
    fn get_nodes_by_depths(&self) -> &Godsnodes<N>;
    fn get_depth(&self) -> usize;
    fn get_root(&self) -> Weak<RwLock<N>>;
    fn parse_from_json(&mut self, raw: &Value);
    fn read_name(&self) -> String;
}

pub struct Godswood<N, T> where N: GodsnodeProto, T: GodswoodProto<N> {
    pub wood: T,
    pub scales: HashMap<usize, f32>,
    pub base_scale: f32,
    pub base_gap: f32,
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
            base_gap: 5.0,
            ph: marker::PhantomData,
        }
    }

    fn calculate_scales(&mut self) {
        println!("Initializing scales for wood");
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
                    // error!("Unexpected weak node lost connection");
                }
            }

            let scale: f32;

            if kids_max <= 1 {
                scale = 1.0;
            } else {
                let angle = PI as f32/ kids_max as f32;
                scale = 1.0 / angle.sin() + 1.0;
            }

            println!("Calculated scale {} for depth {}", scale, i);
            self.scales.insert(i, scale);
        }
        self.scales.insert(depth, 1.0);
        let mut scale = 1.0f32;
        for i in (1..depth).rev() {
            let v = self.scales.get_mut(&i).unwrap();
            *v *= scale;
            scale = *v;
        }
    }

    pub fn render_test(&self) {
        let nodes = self.wood.get_nodes_by_depths();
        let max_depth = self.wood.get_depth();
        let nodes = nodes.read().unwrap();
        for i in 1..max_depth + 1 {
            let items = nodes.get(&i).unwrap();
            // info!("Drawing nodes with depth: {}", i);
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

// ---------------- Sample tree -------------------------------
pub struct TreeProto {
    depth: usize,
    nodes_by_depth: Arc<RwLock<HashMap<usize, Vec<Weak<Node>>>>>,
    root: Weak<Node>,
    store: Arc<Store>,
}

impl TreeProto {
    fn init_nodes(&mut self) {
        let nodes_by_depth = self.nodes_by_depth.clone();
        println!("Initializing nodes for wood");
        if let Some(node) = self.root.upgrade() {
            let mut nodes_by_depth = nodes_by_depth.write().unwrap();

            // Flush nodes queue first
            nodes_by_depth.clear();
            let app_name: String;
            let app_display_name: String;
            let mut app_meta: GodswoodMeta;
            {
                let mut app = node.write().unwrap();
                // info!("Drawing tree architecture of application {}", app.display_name);
                app_display_name = app.display_name.clone();
                app_meta = GodswoodMeta::new();
                app_name = app.name.clone();
                app_meta.path.append(&app.name);
                self.depth = app_meta.path.read_depth();
                app.app_meta_map.insert(app_name.clone(), app_meta.clone());
                self.store.update_index(&app_meta.path.read(), app.id);
                let entry = nodes_by_depth.entry(self.depth).or_insert(Vec::new());
                entry.push(self.root.clone());

            }
            let mut tasks: VecDeque<InitNodeQ<NodeProto>> = VecDeque::new();
            tasks.push_back(InitNodeQ {
                app_meta: app_meta,
                nodes: node.read().unwrap().children.clone(),
            });

            loop {
                let task = tasks.pop_front();
                if task.is_none() {
                    break;
                }
                let task = task.unwrap();

                for child in task.nodes.iter() {
                    if let Some(node) = child.upgrade() {
                        let mut kid_app_meta = task.app_meta.clone();
                        let mut kid = node.write().unwrap();
                        kid_app_meta.path.append(&kid.name);
                        println!("Initializing {}", kid_app_meta.path.read());
                        self.depth = kid_app_meta.path.read_depth();
                        kid.app_meta_map.insert(app_name.clone(), kid_app_meta.clone());
                        self.store.update_index(&kid_app_meta.path.read(), kid.id);
                        let entry = nodes_by_depth.entry(self.depth).or_insert(Vec::new());
                        entry.push(child.clone());

                        tasks.push_back(InitNodeQ {
                            app_meta: kid_app_meta.clone(),
                            nodes: kid.children.clone(),
                        });
                    }
                }
            }

            // info!("Finished drawing tree architecture of application {}", app_display_name);
        }

    }

    // Sample application tree
    // app:
    //   children:
    //     node1:
    //       children:
    //          node3:
    //             children:
    //     node2:
    //       children

    pub fn parse(&mut self, raw:& Value) {
        let root = self.store.add_app_node(&raw);
        self.root = Arc::downgrade(&root);
        if let Some(children) = raw["children"].as_object() {
            if !children.is_empty() {
                TreeProto::parse_children(&root, &children, &mut self.store);
            }
        }
    }

    pub fn parse_children(parent_node: &Arc<Node>, children: & JsonMap, store: &mut Arc<Store>) {
        for (name, raw) in children.iter() {
            let mut node = store.add_node(&raw, name.clone());
            if let Some(sub_children) = raw["children"].as_object() {
                if ! sub_children.is_empty() {
                    TreeProto::parse_children(&mut node, sub_children, store);
                }
            }
            let mut parent = parent_node.write().unwrap();
            parent.add_child(Arc::downgrade(&node));
            let mut child = node.write().unwrap();
            child.add_parent(Arc::downgrade(parent_node));
            // info!("linking parent {} with child {}", parent.name, child.name);
        }
    }
}


impl GodswoodProto<NodeProto> for TreeProto {
    fn default(store: Arc<Godsstore<NodeProto>>) -> Self {
        Self {
            depth: 0, 
            nodes_by_depth: Arc::new(RwLock::new(HashMap::new())),
            root: Weak::new(),
            store: store,
        }
    }

    fn parse_from_json(&mut self, raw: &Value) {
        self.parse(raw);
    }
    fn init_nodes(&mut self) {
        self.init_nodes();
    }
    fn get_nodes_by_depths(&self) -> &Godsnodes<NodeProto> {
        &self.nodes_by_depth
    }
    fn get_depth(&self) -> usize {
        self.depth
    }
    fn get_root(&self) -> Weak<RwLock<NodeProto>> {
        self.root.clone()
    }
    
    fn read_name(&self) -> String {
        let root = self.root.clone();
        root.upgrade().unwrap().read().unwrap().name.clone()
    }

}


