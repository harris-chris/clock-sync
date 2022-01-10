use num::*;
use num::traits::*;

pub trait EventTrigger {
  fn tick(&mut self) -> bool;
}

pub struct NaiveStrategy<A: Unsigned> {
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

pub fn get_naive_strategy<A>(
  freq_base: A,
  freq_target: A,
  count: A,
) -> NaiveStrategy<A> where A: Unsigned {
  let adj_count: A = count * freq_target / freq_base;
  NaiveStrategy { trigger_freq: adj_count, countdown: A::zero() }
}

#[test]
fn test_get_naive_strategy() {
  let ns = get_naive_strategy(1 as u32, 2 as u32, 3 as u32);
  assert_eq!(ns.trigger_freq, 6);
  assert_eq!(ns.countdown, 0);
}

#[test]
fn test_naive_strategy_tick() {
  let mut ns = get_naive_strategy(1 as u32, 2 as u32, 3 as u32);
  assert_eq!(ns.tick(), true);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), true);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
  assert_eq!(ns.tick(), false);
}


