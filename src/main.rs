use color_eyre::{Result, eyre::Context};
use rand::RngExt;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{Event, KeyCode, KeyEvent, read},
    layout::{Constraint, Layout, Position, Spacing},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

#[derive(Copy, Clone)]
struct Guess {
    values: [u32; 4],
}

impl Guess {
    fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self {
            values: [a, b, c, d],
        }
    }
}

#[derive(Copy, Clone)]
struct GuessInfo {
    values: [u32; 2],
}

impl GuessInfo {
    fn new(a: u32, b: u32) -> Self {
        Self { values: [a, b] }
    }
}

struct App {
    run: bool,
    is_finished: bool,
    is_win: bool,
    code: [u32; 4],
    guess_lines: [Guess; 10],
    info_lines: [GuessInfo; 10],
    line_counter: u8,
    guess_counter: u8,
}

impl App {
    fn new() -> Self {
        let mut rng = rand::rng();

        let code = [
            rng.random_range(1..7),
            rng.random_range(1..7),
            rng.random_range(1..7),
            rng.random_range(1..7),
        ];

        Self {
            run: true,
            is_finished: false,
            is_win: false,
            code,
            guess_lines: [Guess::new(0, 0, 0, 0); 10],
            info_lines: [GuessInfo::new(0, 0); 10],
            line_counter: 0,
            guess_counter: 0,
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut app = App::new();

    let app_result = run(&mut app, terminal).context("App loop failed");

    ratatui::restore();
    app_result
}

fn run(mut app: &mut App, mut terminal: DefaultTerminal) -> Result<()> {
    while app.run {
        if terminal.get_frame().area().width < 20 || terminal.get_frame().area().height < 14 {
            terminal.draw(screen_size_warning)?;

            match read().unwrap() {
                Event::Key(event) => {
                    if event.code == KeyCode::Esc {
                        app.run = false;
                    }
                }
                _ => (),
            }
        } else {
            terminal.draw(|frame| render(&mut app, frame))?;

            if app.is_finished {
                terminal.hide_cursor()?;

                match read().unwrap() {
                    Event::Key(event) => match event.code {
                        KeyCode::Enter => {
                            app.is_finished = false;
                            *app = App::new();
                        }
                        KeyCode::Esc => app.run = false,
                        _ => (),
                    },
                    _ => (),
                }
            } else {
                terminal.show_cursor()?;
                let position = cursor_pos_changer(app);
                terminal.set_cursor_position(position)?;

                match read().unwrap() {
                    Event::Key(event) => input_controller(event, &mut app),
                    _ => (),
                }
            }
        }
    }

    Ok(())
}

fn cursor_pos_changer(app: &mut App) -> Position {
    Position {
        x: app.guess_counter as u16 * 2 + 2,
        y: 10 - app.line_counter as u16 + 2,
    }
}

pub fn screen_size_warning(frame: &mut Frame) {
    let lines = vec![
        Line::from(Span::styled("Terminal size too small! ", Style::default())).centered(),
        Line::from(Span::styled(
            format!(
                "Width: {}, Height: {}",
                frame.area().width,
                frame.area().height
            ),
            Style::default(),
        ))
        .centered(),
        Line::from(Span::styled("", Style::default())),
        Line::from(Span::styled(
            "Set your terminal size to minimum",
            Style::default(),
        ))
        .centered(),
        Line::from(Span::styled("Width: 20, Height: 14", Style::default())).centered(),
    ];
    let text = Text::from(lines);
    let p = Paragraph::new(text);

    frame.render_widget(p, frame.area());
}

fn render(app: &mut App, frame: &mut Frame) {
    let horizontal = Layout::horizontal([Constraint::Length(20)]);
    let vertical = Layout::vertical([Constraint::Length(14)]);
    let [area] = vertical.areas(frame.area());
    let [area] = horizontal.areas(area);

    let horizontal = Layout::vertical([Constraint::Length(3), Constraint::Length(12)])
        .spacing(Spacing::Overlap(1));
    let vertical = Layout::horizontal([Constraint::Length(11), Constraint::Length(10)])
        .spacing(Spacing::Overlap(1));

    let [top, bottom] = horizontal.areas(area);
    let [code, info_sht] = vertical.areas(top);
    let [guesses, info] = vertical.areas(bottom);

    let code_p = if app.is_finished {
        Paragraph::new(format!(
            " {} {} {} {} ",
            app.code[0], app.code[1], app.code[2], app.code[3],
        ))
    } else {
        Paragraph::new(" ? ? ? ? ")
    };

    // let code_p = Paragraph::new(format!(
    //     " {} {} {} {} ",
    //     app.code[0], app.code[1], app.code[2], app.code[3],
    // ));

    frame.render_widget(
        code_p.fg(Color::White).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .merge_borders(MergeStrategy::Exact),
        ),
        code,
    );

    let info_sht_p = if app.is_finished {
        if app.is_win {
            let lines = vec![Line::default().spans(["YOU  WIN".light_green()])];
            let text = Text::from(lines);
            Paragraph::new(text)
        } else {
            let lines = vec![Line::default().spans(["YOU LOSE".light_red()])];
            let text = Text::from(lines);
            Paragraph::new(text)
        }
    } else {
        Paragraph::new("  N  X  ")
    };
    frame.render_widget(
        info_sht_p.fg(Color::White).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .merge_borders(MergeStrategy::Exact),
        ),
        info_sht,
    );

    let guess_p = guess_lines(app.line_counter, app.guess_lines);
    frame.render_widget(
        guess_p.fg(Color::White).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .merge_borders(MergeStrategy::Exact),
        ),
        guesses,
    );

    let info_p = info_lines(app.line_counter, app.info_lines);
    frame.render_widget(
        info_p.fg(Color::White).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .merge_borders(MergeStrategy::Exact),
        ),
        info,
    );
}

fn input_controller(event: KeyEvent, app: &mut App) {
    match event.code {
        KeyCode::Char(char) => push_char(app, char),
        KeyCode::Backspace => {
            if app.guess_counter > 0 {
                app.guess_counter -= 1;
                app.guess_lines[app.line_counter as usize].values[app.guess_counter as usize] = 0;
            }
        }
        KeyCode::Enter => {}
        KeyCode::Esc => app.run = false,
        _ => (),
    }
}

fn push_char(app: &mut App, char: char) {
    if char.is_numeric() {
        let num = char.to_digit(10).unwrap();

        if num > 0 && num < 7 {
            app.guess_lines[app.line_counter as usize].values[app.guess_counter as usize] = num;

            if app.guess_counter < 3 {
                app.guess_counter += 1;
            } else {
                if guess_calc(app) {
                    return;
                }
                app.guess_counter = 0;
                app.line_counter += 1;

                if app.line_counter > 9 {
                    app.is_finished = true;
                    app.is_win = false;
                }
            }
        }
    }
}

fn guess_calc(app: &mut App) -> bool {
    let mut corrects = 0;
    let mut pos_corrects = 0;

    for i in 0..4 {
        if app
            .code
            .contains(&app.guess_lines[app.line_counter as usize].values[i as usize])
        {
            corrects += 1
        }

        if app.guess_lines[app.line_counter as usize].values[i as usize] == app.code[i as usize] {
            pos_corrects += 1
        }
    }

    if pos_corrects == 4 {
        app.is_finished = true;
        app.is_win = true;
        return true;
    }

    app.info_lines[app.line_counter as usize].values[0] = corrects - pos_corrects;
    app.info_lines[app.line_counter as usize].values[1] = pos_corrects;
    return false;
}

fn guess_lines(line: u8, guesses: [Guess; 10]) -> Paragraph<'static> {
    let mut lines = vec![];

    for i in (0..10).rev() {
        if i > line {
            lines.push(Line::default().spans([" - - - - "]));
        } else {
            lines.push(Line::default().spans([format!(
                " {} {} {} {} ",
                guesses[i as usize].values[0],
                guesses[i as usize].values[1],
                guesses[i as usize].values[2],
                guesses[i as usize].values[3]
            )]));
        }
    }

    let text = Text::from(lines);
    Paragraph::new(text)
}

fn info_lines(line: u8, infos: [GuessInfo; 10]) -> Paragraph<'static> {
    let mut lines = vec![];

    for i in (0..10).rev() {
        if i >= line {
            lines.push(Line::from(Span::styled("  -  -  ", Style::default())));
        } else {
            lines.push(Line::default().spans([
                format!("  {} ", infos[i as usize].values[0]).blue(),
                format!(" {}  ", infos[i as usize].values[1]).green(),
            ]));
        }
    }

    let text = Text::from(lines);
    Paragraph::new(text)
}
