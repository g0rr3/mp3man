use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Error};
use std::{env, fmt};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::{Frame, Terminal};

struct App<'a> {
    current: &'a String,
    sink: Sink,
    running: bool,
}

impl<'a> fmt::Debug for App<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("current", &self.current)
            .field("running", &self.running)
            .finish()
    }
}

impl<'a> App<'a> {
    fn stutdown(&mut self) {
        self.running = false;
    }
}

const DEFAULT_VOLUME: f32 = 0.5;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let (_stram, stream_handle) = OutputStream::try_default().unwrap();

    let mut app = App {
        current: &args[1],
        sink: Sink::try_new(&stream_handle).unwrap(),
        running: true,
    };
    app.sink.set_volume(DEFAULT_VOLUME);

    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    while app.running {
        terminal.draw(|f| ui(f))?;
        listen_keys(&mut app);
    }

    terminal.clear()?;
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let right_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);
    let paragraph = Paragraph::new("aaa")
        .alignment(tui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    let file_list = Block::default().title("Files").borders(Borders::ALL);
    f.render_widget(file_list, chunks[0]);
    f.render_widget(paragraph, right_chunk[0]);
    f.render_widget(block, right_chunk[1]);
}

fn listen_keys(app: &mut App) {
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                app.stutdown();
                return;
            }
            Key::Char('+') => {
                if app.sink.volume() < 1 {
                    app.sink.set_volume(app.sink.volume() + 0.1);
                }
            }
            Key::Char('-') => {
                if app.sink.volume() > 0 {
                    app.sink.set_volume(app.sink.volume() - 0.1);
                }
            }
            Key::Char(' ') => {
                if app.sink.is_paused() {
                    app.sink.play();
                } else {
                    app.sink.pause();
                }
            }
            Key::Char('p') => play(app),
            _ => {}
        }
    }
}

fn play(app: &App) {
    let file = BufReader::new(File::open(app.current).unwrap());
    let source = Decoder::new(file).unwrap();
    app.sink.append(source)
}
