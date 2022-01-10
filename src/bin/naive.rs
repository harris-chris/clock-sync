use clocksync_lib as cs_lib;
use structopt::StructOpt;
use cs_lib::EventTrigger;

fn main() {
  env_logger::init();
  let opt = cs_lib::Opt::from_args();
  let mut ns = cs_lib::get_naive_strategy(
    opt.b_freq, opt.t_freq, opt.triggers_every
  );
  for i in 0..opt.o_num {
    let trigger = ns.tick();
    print!("{}:{}\t", i, trigger);
  }
}
