use std::io::{Stdout, Write};
use crate::frame::Frame;
use crossterm::{
  QueueableCommand,
  style::{ SetBackgroundColor,Color},
  terminal::{Clear, ClearType },
  cursor::MoveTo,
};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
  if force {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();

  }
  for (x, col) in curr_frame.iter().enumerate() {
    for (y, s) in col.iter().enumerate() {
      if *s != last_frame[x][y] || force {
          stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
          println!("{}",*s);
      }
    }
  }
  stdout.flush().unwrap();
}