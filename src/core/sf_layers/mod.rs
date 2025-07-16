use super::sf_events::Eventable;

pub struct LayerStack {
    pub layers: Vec<Box<dyn Layer>>,
    layer_insert_index: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            layer_insert_index: 0,
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.layer_insert_index, layer);
        self.layer_insert_index += 1;
    }
    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }

    pub fn pop_layer(&mut self) {
        if self.layer_insert_index > 0 {
            self.layers.remove(self.layer_insert_index);
            self.layer_insert_index -= 1;
        }
    }
}

pub trait Layer {
    fn get_name(&mut self) -> &String;
    fn on_attach(&mut self);
    fn on_detach(&mut self);
    fn on_update(&mut self);
    fn on_event(&mut self, event: &dyn Eventable);
}
