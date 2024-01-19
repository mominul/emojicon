pub trait Check<T> {
    fn checked_push(&mut self, value: T);
}

impl<T: PartialEq> Check<T> for Vec<T> {
    fn checked_push(&mut self, value: T) {
        if !self.contains(&value) {
            self.push(value);
        }
    }
}