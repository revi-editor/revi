use crate::{
    application::App,
    layout::{Pos, Rect, Size},
};
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand, Result,
};
use std::io::Stdout;
use std::{io::Write, time::Duration};

fn render_app<A>(w: &mut Stdout, app: &mut A) -> Result<()>
where
    A: App,
{
    w.queue(Hide)?;
    if let Some(Pos { x, y }) = app.cursor_pos() {
        w.queue(MoveTo(x, y))?;
    }
    if let Some(cs) = app.cursor_shape() {
        w.queue(cs)?;
    }
    let widgets = app.view();
    let width = widgets.width();
    let height = widgets.height();
    let app_size = Size { width, height };
    let app_pos = Pos { x: 0, y: 0 };
    w.queue(SavePosition)?;
    widgets.draw(w, Rect::with_position(app_pos, app_size));
    w.queue(RestorePosition)?;
    if app.cursor_shape().is_some() {
        w.queue(Show)?;
    }
    w.flush()?;
    Ok(())
}

fn update<A>(app: &mut A, message: A::Message)
where
    A: App,
{
    let Some(message) = app.update(message) else {
        return;
    };
    update(app, message);
}

pub fn run<A>(app: &mut A) -> Result<()>
where
    A: App,
{
    let mut writer = std::io::stdout();
    writer.queue(EnterAlternateScreen)?;
    writer.queue(SavePosition)?;
    writer.queue(Hide)?;
    enable_raw_mode()?;
    writer.flush()?;
    let mut subscriptions = app.subscription();
    render_app(&mut writer, app)?;
    while app.quit() {
        if event::poll(Duration::from_millis(50)).unwrap_or(false) {
            let event = event::read()?;
            for sub in subscriptions.0.iter() {
                let Some(message) = sub(event.clone()) else {
                    continue;
                };
                update(app, message);
            }
            render_app(&mut writer, app)?;
            subscriptions = app.subscription();
        }
    }

    disable_raw_mode()?;
    writer.queue(LeaveAlternateScreen)?;
    writer.queue(RestorePosition)?;
    writer.queue(Show)?;
    writer.flush()?;
    Ok(())
}
