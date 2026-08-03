pub trait Generator {
    fn get_next_value(&mut self) -> f32;
}
