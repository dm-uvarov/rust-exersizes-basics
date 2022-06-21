use std::{
    error::Error,
    io,
    time::Duration,
    
};
use rusty_audio::Audio;
use crossterm::{
    cursor::{Hide,Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    event::{self, Event, KeyCode}
};


fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode","sounds/explode.wav");
    audio.add("lose","sounds/lose.wav");
    audio.add("move","sounds/move.wav");
    audio.add("pew","sounds/pew.wav");
    audio.add("startup","sounds/startup.wav");
    audio.add("win","sounds/win.wav");

    audio.play("startup");

    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; //new screen similar to wim
    stdout.execute(Hide)?; // no cursor

    // game loop
    'gameloop: loop{
        //input 
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
               match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    audio.play("lose");
                    break 'gameloop;
                },
                _ => {} //ignore other keys
               }
            }
        }
    }

    // cleanup 
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())  // return 
}
