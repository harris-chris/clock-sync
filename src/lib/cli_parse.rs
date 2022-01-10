use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cs_parse")]
pub struct Opt {
  #[structopt(short = "b", long = "base_freq")]
  pub b_freq: u32,

  #[structopt(short = "t", long = "target_freq")]
  pub t_freq: u32,

  #[structopt(short = "e", long = "triggers_every")]
  pub triggers_every: u32,

  #[structopt(short = "o", long = "output_num", default_value = "10")]
  pub o_num: u32,
}

