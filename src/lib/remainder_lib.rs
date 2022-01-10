use num::traits::{Unsigned, NumAssignOps};
use crate::cs_trait::*;

pub struct RemainderStrategy<A: Unsigned> {
  trigger_freq: A,
  countdown: A,
}

impl<A> EventTrigger for NaiveStrategy<A>
  where A: Unsigned + NumAssignOps + Copy {
  fn tick(&mut self) -> bool {
    let trigger = self.countdown == A::zero();
    self.countdown = if trigger { self.trigger_freq } else { self.countdown };
    self.countdown -= A::one();
    trigger
  }
}
