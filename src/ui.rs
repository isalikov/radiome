use crate::{
    app::{App, Category},
    player::PlayerState,
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph},
};

const BG: Color = Color::Rgb(10, 12, 20);
const FG: Color = Color::Rgb(216, 222, 239);
const MUTED: Color = Color::Rgb(111, 121, 149);
const EDGE: Color = Color::Rgb(39, 46, 65);
const CYAN: Color = Color::Rgb(53, 235, 221);
const PINK: Color = Color::Rgb(244, 94, 191);
const SELECTED: Color = Color::Rgb(40, 27, 54);

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    frame.render_widget(Block::default().style(Style::default().bg(BG).fg(FG)), area);
    if area.width < 44 || area.height < 12 {
        frame.render_widget(
            Paragraph::new("Minimum 44 × 12\nq  quit").style(Style::default().fg(MUTED)),
            area.inner(Margin::new(1, 1)),
        );
        return;
    }
    let [body, footer] = Layout::vertical([Constraint::Min(1), Constraint::Length(4)])
        .areas(area.inner(Margin::new(2, 1)));
    let category_width = if area.width >= 90 { 25 } else { 18 };
    let [sidebar, right] =
        Layout::horizontal([Constraint::Length(category_width), Constraint::Min(1)]).areas(body);
    categories(frame, app, sidebar);
    let right = right.inner(Margin::new(1, 0));
    if app.show_history && body.height >= 12 {
        let [stations_area, history_area] = Layout::vertical([
            Constraint::Min(6),
            Constraint::Length((body.height / 3).clamp(5, 8)),
        ])
        .areas(right);
        stations(frame, app, stations_area);
        history(frame, app, history_area);
    } else {
        stations(frame, app, right);
    }
    playback(frame, app, footer);
    if app.help {
        help(frame, area, app.help_scroll);
    }
}

fn panel(title: &str) -> Block<'_> {
    Block::default()
        .title(Line::from(Span::styled(title, Style::default().fg(MUTED))))
        .borders(Borders::TOP)
        .border_style(Style::default().fg(EDGE))
        .padding(Padding::new(0, 0, 0, 0))
}

fn categories(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = panel("")
        .borders(Borders::RIGHT)
        .padding(Padding::new(0, 1, 0, 0));
    let items: Vec<_> = app
        .categories
        .iter()
        .map(|category| {
            let line = Line::raw(category.title().to_owned());
            if matches!(category, Category::Favorites) {
                ListItem::new(vec![line, Line::raw("")])
            } else {
                ListItem::new(line)
            }
        })
        .collect();
    let selected = Style::default().fg(PINK).add_modifier(Modifier::BOLD);
    frame.render_stateful_widget(
        List::new(items)
            .block(block)
            .highlight_style(selected)
            .highlight_symbol("▏"),
        area,
        &mut app.category,
    );
}

fn stations(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = panel("").borders(Borders::NONE);
    let inner = block.inner(area);
    let visible = app.visible();
    if visible.is_empty() {
        let text = if app.loading {
            "Loading…"
        } else if app.error.is_some() && app.catalog.is_none() {
            "Stations unavailable · r retry"
        } else if matches!(app.current_category(), Category::Favorites) {
            "No favorites · f to add"
        } else {
            "No stations"
        };
        frame.render_widget(block, area);
        frame.render_widget(
            Paragraph::new(text).style(Style::default().fg(MUTED)),
            inner,
        );
        return;
    }
    let items: Vec<_> = visible
        .iter()
        .map(|station| {
            let is_current = app.now.as_ref().is_some_and(|now| now.id == station.id)
                && !matches!(app.playback.state, PlayerState::Idle | PlayerState::Error);
            let marker = if is_current {
                match app.playback.state {
                    PlayerState::Paused => "Ⅱ ",
                    PlayerState::Buffering => "· ",
                    _ => "▶ ",
                }
            } else {
                "  "
            };
            let line = Line::from(vec![
                Span::styled(marker, Style::default().fg(CYAN)),
                Span::styled(
                    station.title.clone(),
                    if is_current {
                        Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(FG)
                    },
                ),
                Span::styled(
                    if app.settings.favorites.contains(&station.id)
                        && !matches!(app.current_category(), Category::Favorites)
                    {
                        "  +"
                    } else {
                        ""
                    },
                    Style::default().fg(PINK),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();
    frame.render_stateful_widget(
        List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(SELECTED))
            .highlight_symbol("▏"),
        area,
        &mut app.stations,
    );
}

fn history(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = panel("History ");
    let inner = block.inner(area);
    if app.history.len() <= 1 {
        let text = if let Some(error) = &app.history_error {
            error.as_str()
        } else if app.history_loading {
            "Loading…"
        } else if app.now.is_none() {
            ""
        } else {
            "No track history"
        };
        frame.render_widget(block, area);
        frame.render_widget(
            Paragraph::new(text).style(Style::default().fg(MUTED)),
            inner,
        );
        return;
    }
    let items: Vec<_> = app
        .history
        .iter()
        .skip(1)
        .map(|track| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{}  ", track.time_formatted),
                    Style::default().fg(MUTED),
                ),
                Span::styled(track.artist.clone(), Style::default().fg(FG)),
                Span::styled(format!(" — {}", track.song), Style::default().fg(MUTED)),
            ]))
        })
        .collect();
    frame.render_widget(List::new(items).block(block), area);
    if app.history_error.is_some() && inner.height > 0 {
        frame.render_widget(
            Paragraph::new("History out of date · r retry").style(Style::default().fg(PINK)),
            Rect::new(inner.x, inner.bottom() - 1, inner.width, 1),
        );
    }
}

fn playback(frame: &mut Frame, app: &App, area: Rect) {
    let level = if app.playback.state == PlayerState::Playing {
        app.playback.level
    } else {
        0.0
    };
    frame.render_widget(
        Paragraph::new(meter(area.width, level)),
        Rect::new(area.x, area.y, area.width, 1),
    );
    let [playing, volume] = Layout::horizontal([Constraint::Min(1), Constraint::Length(5)])
        .areas(Rect::new(area.x, area.y + 1, area.width, 1));
    let symbol = match app.playback.state {
        PlayerState::Idle => "○",
        PlayerState::Buffering => "·",
        PlayerState::Playing => "▶",
        PlayerState::Paused => "Ⅱ",
        PlayerState::Error => "!",
    };
    let name = app
        .now
        .as_ref()
        .map(|station| station.title.as_str())
        .unwrap_or("Stopped");
    frame.render_widget(
        Paragraph::new(format!("{symbol}  {name}")).style(Style::default().fg(CYAN)),
        playing,
    );
    frame.render_widget(
        Paragraph::new(format!("{}%", app.settings.volume))
            .right_aligned()
            .style(Style::default().fg(MUTED)),
        volume,
    );
    let track = if let Some(error) = &app.playback.error {
        error.clone()
    } else if let Some(track) = app.history.first() {
        format!("{} — {}", track.artist, track.song)
    } else if app.history_loading {
        "Loading track…".into()
    } else {
        String::new()
    };
    frame.render_widget(
        Paragraph::new(track).style(Style::default().fg(if app.playback.error.is_some() {
            PINK
        } else {
            MUTED
        })),
        Rect::new(area.x, area.y + 2, area.width, 1),
    );
    let status = app.error.as_deref().unwrap_or("");
    frame.render_widget(
        Paragraph::new(status).style(Style::default().fg(PINK)),
        Rect::new(area.x, area.y + 3, area.width, 1),
    );
}

// One thin stroke; its endpoint follows the current audio level, with no history or scrolling.
// The terminal's font determines the physical stroke thickness.
fn meter(width: u16, level: f64) -> Line<'static> {
    let level = if level.is_finite() {
        level.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let active = (level * f64::from(width)).round() as usize;
    Line::from(vec![
        Span::styled("─".repeat(active), Style::default().fg(CYAN)),
        Span::styled(
            "─".repeat(usize::from(width) - active),
            Style::default().fg(EDGE),
        ),
    ])
}

fn help(frame: &mut Frame, area: Rect, scroll: u16) {
    let width = 52.min(area.width.saturating_sub(4));
    let height = 17.min(area.height);
    let popup = Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    );
    frame.render_widget(Clear, popup);
    let lines = [
        ("↑ ↓ / j k", "select"),
        ("Tab / Shift Tab", "next / previous category"),
        ("Enter", "play station"),
        ("Space / F8", "play / pause"),
        ("F7 / F9", "previous / next station"),
        ("− / =", "player volume"),
        ("f", "toggle favorite"),
        ("Esc", "close"),
        ("i", "toggle history"),
        ("PgUp PgDn Home End", "scroll"),
        ("r", "refresh"),
        ("s", "stop"),
        ("q / Ctrl C", "quit"),
    ]
    .into_iter()
    .map(|(keys, label)| {
        Line::from(vec![
            Span::styled(format!("{keys:<19}"), Style::default().fg(CYAN)),
            Span::raw(label),
        ])
    })
    .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(lines)
            .scroll((
                scroll.min(13u16.saturating_sub(height.saturating_sub(4))),
                0,
            ))
            .style(Style::default().fg(FG).bg(BG))
            .block(
                Block::default()
                    .title(" Keys ")
                    .title_bottom(if height < 17 {
                        " ↑↓  ? / Esc "
                    } else {
                        " ? / Esc "
                    })
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(PINK))
                    .padding(Padding::uniform(1)),
            ),
        popup,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api::Track, app::tests::fixture};
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn favorite_marker_only_appears_outside_favorites() {
        let mut app = fixture();
        app.settings.favorites.insert(1);
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();
        let text = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();
        assert!(text.contains("Record  +"));
        assert!(!text.contains(['♥', '♡']));
        assert!(!text.contains("radiome"));
        app.category.select(Some(1));
        terminal.draw(|frame| render(frame, &mut app)).unwrap();
        let text = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();
        assert!(text.contains("Favorites"));
        assert!(text.contains("Record"));
        assert!(!text.contains('+'));
    }

    #[test]
    fn meter_is_one_thin_stroke_with_an_instantaneous_endpoint() {
        for (level, expected) in [(0.0, 0), (0.25, 10), (0.75, 30), (1.0, 40), (f64::NAN, 0)] {
            let mut terminal = Terminal::new(TestBackend::new(40, 1)).unwrap();
            terminal
                .draw(|frame| frame.render_widget(Paragraph::new(meter(40, level)), frame.area()))
                .unwrap();
            let cells = terminal.backend().buffer().content();
            assert!(cells.iter().all(|cell| cell.symbol() == "─"));
            assert_eq!(
                cells.iter().filter(|cell| cell.fg == CYAN).count(),
                expected
            );
        }
    }

    #[test]
    fn renders_unicode_empty_states_and_small_terminals() {
        for (width, height) in [(120, 38), (80, 24), (44, 12), (20, 6), (1, 1)] {
            let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
            let mut app = fixture();
            app.now = app.selected_station().cloned();
            app.playback.state = PlayerState::Playing;
            app.history = (0..12)
                .map(|index| Track {
                    id: index,
                    artist: "Артист".into(),
                    song: "Очень длинное название трека 音楽 🎵".repeat(4),
                    image100: String::new(),
                    image200: String::new(),
                    time_formatted: "12:34".into(),
                })
                .collect();
            terminal.draw(|frame| render(frame, &mut app)).unwrap();
            if width >= 80 {
                let buffer = terminal.backend().buffer();
                let text = buffer
                    .content()
                    .iter()
                    .map(|cell| cell.symbol())
                    .collect::<String>();
                assert!(!text.contains("radiome"));
                assert!(text.contains("Favorites"));
                assert!(text.contains("Record"));
                assert!(
                    buffer
                        .content()
                        .iter()
                        .any(|cell| cell.fg == CYAN && cell.symbol() == "▶")
                );
            }
            app.help = true;
            terminal.draw(|frame| render(frame, &mut app)).unwrap();
            app.help = false;
            app.category.select(Some(1));
            terminal.draw(|frame| render(frame, &mut app)).unwrap();
        }
    }

    #[test]
    fn scrolling_reaches_last_station() {
        let mut app = fixture();
        let template = app.catalog.as_ref().unwrap().stations[0].clone();
        app.catalog.as_mut().unwrap().stations = (0..100)
            .map(|id| {
                let mut station = template.clone();
                station.id = id;
                station.title = format!("Station {id:03}");
                station
            })
            .collect();
        app.stations.select(Some(99));
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();
        let text = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();
        assert!(text.contains("Station 099"));
        assert!(!text.contains("Station 000"));
    }
}
