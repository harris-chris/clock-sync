pub trait EventTrigger {
  fn tick(&mut self) -> bool;
}
