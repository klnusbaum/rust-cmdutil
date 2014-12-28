use progress_bar::ProgressAmount;

pub mod progress_bar {

  use std::io::process::{Command,ProcessOutput, ProcessExit};

  pub struct ProgressAmount(f32);

  impl ProgressAmount {
    pub fn new(amount: f32, total: f32) -> ProgressAmount {
      if amount > total {
        return ProgressAmount(1f32)
      } else {
        ProgressAmount(amount / total)
      }
    }
  }

  impl Copy for ProgressAmount {

  }

  pub fn display_progress(progress: ProgressAmount) -> () {
    let term_width = match get_term_width() {
      Some(a) => a,
      _ => 20
    };
    let bar_length = calc_bar_length(progress, term_width as f32);
    print!("|");
    for _ in range(0, bar_length-1) {
      print!("=");
    }
    print!(">");
    for _ in range(0, term_width-bar_length) {
      print!(" ");
    }
    print!("|\r");
  }

  fn get_term_width() -> Option<u16> {
    match Command::new("tput").arg("cols").output() {
      Ok(ProcessOutput { error: _, output: out, status: exit}) => process_tput_output(out, exit),
      Err(_) => None
    }
  }

  fn process_tput_output(out: Vec<u8>, exit: ProcessExit) -> Option<u16> {
    if exit.success() {
      match String::from_utf8(out) {
        Ok(res) => res.as_slice().trim().parse::<u16>(),
        Err(_) => None
      }
    } else {
      None
    }
  }

  fn calc_bar_length(progress: ProgressAmount, term_width: f32) -> u16 {
    let ProgressAmount(f32_progress) = progress;
    (f32_progress * term_width) as u16
  }
}



fn main() {
  progress_bar::display_progress(ProgressAmount::new(30f32,100f32));
  println!("");
}
