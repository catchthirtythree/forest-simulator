use crate::grid::Grid;

pub trait Entity: EntityClone {
    fn get_symbol(&self) -> &str;
    fn update(&self, idx: usize, grid: &mut Grid<Option<Box<dyn Entity>>>);
}

pub trait EntityClone {
    fn clone_box(&self) -> Box<dyn Entity>;
}

impl<T> EntityClone for T where T: 'static + Entity + Clone {
    fn clone_box(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Entity> {
    fn clone(&self) -> Box<dyn Entity> {
        self.clone_box()
    }
}
