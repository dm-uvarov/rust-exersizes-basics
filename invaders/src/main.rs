use std::{
    error::Error,
    io,
    time::Duration, sync::mpsc, thread,
    
};
use invaders::{frame::{self, new_frame, Drawable}, render, player::Player};
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

    // render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();// for real world use crosbeam channel
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x)=> x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });



    // game loop
    let mut player = Player::new();

    'gameloop: loop{
        // pure frame init 
        let mut curr_frame = new_frame();

        //input 
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
               match key_event.code {
                KeyCode::Left => player.move_left(),
                KeyCode::Right => player.move_right(),
                KeyCode::Esc | KeyCode::Char('q') => {
                    audio.play("lose");
                    break 'gameloop;
                },
                _ => {} //ignore other keys
               }
            }
        }

        // draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        // mismatch speed of loops
        thread::sleep(Duration::from_millis(1));
    }

    // cleanup 
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())  // return 
}
