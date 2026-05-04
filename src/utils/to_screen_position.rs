pub trait ToScreenPosition {
    fn to_coords(&self) -> (usize, usize);
}

impl ToScreenPosition for (usize, usize) {
    fn to_coords(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl ToScreenPosition for [usize; 2] {
    fn to_coords(&self) -> (usize, usize) {
        (self[0], self[1])
    }
}
