use super::sf_events::{Event, Eventable};

pub struct LayerStack<'a> {
    layers: Vec<&'a dyn Layer>,
    layer_insert_index: usize,
}

impl<'a> LayerStack<'a> {
    pub fn push_layer(&mut self, layer: &'a dyn Layer) {
        self.layers.insert(self.layer_insert_index, layer);
        self.layer_insert_index += 1;
    }
    pub fn push_overlay(&mut self, layer: &'a dyn Layer) {
        self.layers.push(layer);
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
