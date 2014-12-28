pub mod progress_bar {

  use std::io::process::{Command,ProcessOutput, ProcessExit};

  pub fn display_progress(progress: u16) -> () {
    let term_width = match get_term_width() {
      Some(a) => a,
      _ => 20
    };
    let bar_length = calc_bar_length(progress as f32, term_width as f32);
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

  fn calc_bar_length(progress: f32, term_width: f32) -> u16 {
    ((progress / 100f32) * term_width) as u16
  }
}

fn main() {
  progress_bar::display_progress(30);
  println!("");
}
