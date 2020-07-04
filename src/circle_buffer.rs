const BUFFER_MAX: usize = 20;

#[derive(Debug)]
pub struct CircleBuffer<T> {
    pub values: Vec<T>,
    pub current_item: usize,
}

impl<T: Default + Clone> CircleBuffer<T> {
    pub fn new() -> Self {
        CircleBuffer {
            values: vec![T::default(); BUFFER_MAX],
            current_item: 0,
        }
    }
}

impl<T> CircleBuffer<T> {
    pub fn set_next(&mut self, val: T) {
        self.values[self.current_item] = val;
        self.current_item = if self.current_item == BUFFER_MAX - 1 {
            0
        } else {
            self.current_item + 1
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values.iter()
    }
}

//impl<T> IntoIterator for CircleBuffer<T> {
//    type Item = T;
//    type IntoIter = std::vec::IntoIter<Self::Item>;
//    fn into_iter(self) -> Self::IntoIter {
//        self.values.into_iter()
//    }
//}
