use num::traits::{Float, NumAssignOps};
use std::fmt::Display;
use log::{info, trace, warn};
use crate::cs_trait::EventTrigger;

pub struct RemainderStrategy<A: Float> {
  trigger_freq: A,
  countdown: A,
}

impl<A> EventTrigger for RemainderStrategy<A>
  where A: Float + NumAssignOps + Copy + Display {
  fn tick(&mut self) -> bool {
    let trigger = self.countdown <= A::zero();
    if trigger {
      self.countdown += self.trigger_freq
    }
    self.countdown -= A::one();
    info!("countdown: {}, trigger_freq: {}", self.countdown, self.trigger_freq);
    trigger
  }
}

pub fn get_remainder_strategy<A>(
  freq_base: A,
  freq_target: A,
  count: A,
) -> RemainderStrategy<A> where A: Float {
  let adj_count: A = count * freq_target / freq_base;
  RemainderStrategy { trigger_freq: adj_count, countdown: A::zero() }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_get_remainder_strategy() {
    let ns = get_remainder_strategy(1., 2., 3.);
    assert_eq!(ns.trigger_freq, 6.);
    assert_eq!(ns.countdown, 0.);
  }

  #[test]
  fn test_remainder_strategy_tick_divisible() {
    let mut ns = get_remainder_strategy(1., 2., 3.);
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

  #[test]
  fn test_remainder_strategy_tick_indivisible() {
    env_logger::init();
    // 3/4 should round up to 1, so trigger frequency will remain unchanged
    let mut ns = get_remainder_strategy(3., 4., 2.);
    assert_eq!(ns.tick(), true);
    assert_eq!(ns.tick(), false);   // cd = 2.67 - 1.         = 1.67
    assert_eq!(ns.tick(), false);   // cd = 1.67 - 1.         = 0.67
    assert_eq!(ns.tick(), true);    // cd = 0.67 - 1.         = -0.34
    assert_eq!(ns.tick(), false);   // cd = -0.34 + 2.67 - 1. = 1.33
    assert_eq!(ns.tick(), false);   // cd = 1.33 - 1.         = 0.33
    assert_eq!(ns.tick(), true);    // cd = 0.33 - 1.         = -0.66
    assert_eq!(ns.tick(), false);   // cd = -0.66 + 2.67 - 1. = 0.99
    assert_eq!(ns.tick(), true);    // cd = 0.99 - 1.         = -0.01
    assert_eq!(ns.tick(), false);   // cd = -0.01 + 2.67 - 1. = 1.67
  }
}
