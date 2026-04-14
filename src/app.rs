use std::{io::{Stdout, stdout}, time::{Duration, Instant}, thread};
use crossterm::{cursor, event::{self, Event, KeyCode, KeyModifiers}, execute};

pub struct App<T> {
    should_exit: bool,
    pub data: T,
    pub stdout: Stdout
}

impl<T> App<T> {
    pub fn new(data: T) -> App<T> {
        App::<T> {
            should_exit: false,
            data: data,
            stdout: stdout()
        }
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    fn poll_events(&mut self, event_handler: fn(&mut App<T>, Event) -> Result<(), String>) -> Result<(), String> {
        let is_event = event::poll(Duration::from_millis(1)).unwrap_or(false);
        if !is_event {return Ok(());}
        let Ok(event) = event::read() else {return Ok(());};

        if let Event::Key(key) = event {
            if key.is_press() && key.modifiers == KeyModifiers::CONTROL && (key.code == KeyCode::Char('c') || key.code == KeyCode::Char('q')) {
                self.exit();
            }
        }
        event_handler(self, event)
    }

    pub fn main(&mut self, action: fn(&mut App<T>) -> Result<(), String>, event_handler: fn(&mut App<T>, Event) -> Result<(), String>) -> Result<(), String> {
        execute!(
            self.stdout,
            crossterm::terminal::EnterAlternateScreen,
            cursor::Hide
        ).unwrap();
        crossterm::terminal::enable_raw_mode().expect("Unable to enable raw mode");

        let mut result: Result<(), String> = Ok(());

        let updates_per_second = 32;
        let frame_delay = Duration::from_nanos(1_000_000_000u64 / updates_per_second as u64);

        while !self.should_exit {
            let frame_start = Instant::now();

            let poll_result = self.poll_events(event_handler);
            if poll_result.is_err() {
                result = poll_result;
                break;
            }

            let loop_result = action(self);
            if loop_result.is_err() {
                result = loop_result;
                break;
            }

            let elapsed = frame_start.elapsed();
            if elapsed < frame_delay {
                thread::sleep(frame_delay - elapsed);
            }
        }

        crossterm::terminal::disable_raw_mode().unwrap();
        execute!(
            self.stdout,
            crossterm::terminal::LeaveAlternateScreen,
            cursor::Show
        ).unwrap();

        result
    }
}
