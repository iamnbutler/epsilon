pub trait Completable {
    fn completed(&self) -> bool;
    fn set_completed(&mut self, completed: bool);
}
