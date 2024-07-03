use std::io::{self, stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor,
};
use crate::evaluator::tokenizer::Tokenizer;

mod commands;
mod evaluator;


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    fn draw_prompt(stdout: &mut impl Write) -> io::Result<()> {
        write!(stdout, "\rλ> ")?;
        stdout.flush()?;
        Ok(())
    }

    write!(stdout, "{}", commands::commandHelp())?;
    stdout.flush()?;
    write!(stdout, "\r\n")?;

    draw_prompt(&mut stdout)?;

    let mut input = String::new();

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Enter => {
                        if input.is_empty() {
                            write!(stdout, "\n")?;
                            draw_prompt(&mut stdout)?;
                            continue;
                        }
                        match input.as_str() {
                            "exit" => {
                                break;
                            }
                            "clear" => {
                                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0))?;
                                input.clear();
                                stdout.flush()?;
                                draw_prompt(&mut stdout)?;
                            }
                            "help" => {
                                input.clear();
                                stdout.flush()?;
                                write!(stdout, "\r\n")?;
                                write!(stdout, "{}", commands::commandHelp())?;
                                write!(stdout, "\n")?;
                                stdout.flush()?;
                                draw_prompt(&mut stdout)?;
                            }
                            _ => {
                                let mut tokenizer = Tokenizer { 
                                    line: input.clone(),
                                    index: 0,
                                    tokens: vec![],
                                    current: '\0'
                                };

                                tokenizer.tokenize();
                        
                                write!(stdout, "\r\n")?;
                                stdout.flush()?;
                                write!(stdout,"{}",commands::commandFinder(&tokenizer.tokens))?;
                                write!(stdout, "\n")?;
                                stdout.flush()?;

                                input.clear();
                                draw_prompt(&mut stdout)?;
                            }
                        } 
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            execute!(stdout, Clear(ClearType::CurrentLine))?;
                            draw_prompt(&mut stdout)?;
                            write!(stdout, "{}", input)?;
                            stdout.flush()?;
                        }
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        write!(stdout, "{}", c)?;
                        stdout.flush()?;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
