use clocksync_lib as cs_lib;
use clocksync_lib::EventTrigger;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let output_count = args[1].parse::<u32>().unwrap();
  let mut ns = cs_lib::get_naive_strategy(1 as u32, 2 as u32, 3 as u32);
  for i in 0..output_count {
    let trigger = ns.tick();
    print!("{}:{}\t", i, trigger);
  }
}
