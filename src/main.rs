mod api;
mod error;
mod json;
mod player;

use std::io::{self, Write};

use api::{Client, Station};
use error::{Error, Result};
use player::{Player, PlayerState};

fn main() {
    if let Err(err) = run() {
        eprintln!("radiome: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let client = Client::new();
    let mut player = Player::new();
    let mut app = App::load(&client)?;

    println!("radiome");
    println!("Type 'help' to see commands.");

    loop {
        player.update();
        app.player_state = player.state();
        app.player_error = player.last_error().map(ToString::to_string);
        app.player_url = player.current_url().map(ToString::to_string);
        app.render();

        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            println!();
            break;
        }

        let command = input.trim();
        if command.is_empty() {
            continue;
        }

        match app.handle_command(command, &client, &mut player) {
            Ok(Exit::Continue) => {}
            Ok(Exit::Quit) => break,
            Err(err) => {
                app.message = format!("error: {err}");
            }
        }
    }

    player.stop();
    Ok(())
}

enum Exit {
    Continue,
    Quit,
}

struct App {
    stations: Vec<Station>,
    filter: String,
    selected: usize,
    message: String,
    player_state: PlayerState,
    player_error: Option<String>,
    player_url: Option<String>,
}

impl App {
    fn load(client: &Client) -> Result<Self> {
        let stations = client.get_stations()?;
        Ok(Self {
            stations,
            filter: String::new(),
            selected: 0,
            message: String::from("Loaded stations"),
            player_state: PlayerState::Idle,
            player_error: None,
            player_url: None,
        })
    }

    fn render(&self) {
        clear_screen();
        println!("radiome - minimal Radio Record player");
        println!(
            "stations: {} | filter: {}",
            self.stations.len(),
            if self.filter.is_empty() {
                "<none>"
            } else {
                &self.filter
            }
        );
        println!(
            "player: {}{}",
            match self.player_state {
                PlayerState::Idle => "idle",
                PlayerState::Playing => "playing",
                PlayerState::Error => "error",
            },
            self.player_error
                .as_ref()
                .map(|msg| format!(" ({msg})"))
                .unwrap_or_default()
        );
        if let Some(url) = &self.player_url {
            println!("url: {url}");
        }
        println!("message: {}", self.message);
        println!();
        let filtered = self.filtered_stations();
        println!("visible stations: {}", filtered.len());
        for (index, station) in filtered.iter().take(20).enumerate() {
            let marker = if index == self.selected { ">" } else { " " };
            let url = station.stream_url().unwrap_or("<no stream>");
            println!(
                "{} {:>3}. {:<28} {:<24} {}",
                marker,
                index + 1,
                truncate(&station.title, 28),
                truncate(&station.tooltip, 24),
                url
            );
        }
        if filtered.len() > 20 {
            println!("... and {} more", filtered.len() - 20);
        }
        println!();
        println!(
            "commands: help | list | search <text> | clear | play [n|text] | stop | now | refresh | up | down | quit"
        );
    }

    fn handle_command(
        &mut self,
        command: &str,
        client: &Client,
        player: &mut Player,
    ) -> Result<Exit> {
        let mut parts = command.split_whitespace();
        let head = parts.next().unwrap_or("");
        let tail = parts.collect::<Vec<_>>().join(" ");

        match head {
            "q" | "quit" | "exit" => Ok(Exit::Quit),
            "help" | "?" => {
                self.message = String::from(
                    "commands: help | list | search <text> | clear | play [n|text] | stop | now | refresh | up | down | quit",
                );
                Ok(Exit::Continue)
            }
            "list" => Ok(Exit::Continue),
            "search" | "filter" => {
                self.filter = tail;
                self.selected = 0;
                self.message = if self.filter.is_empty() {
                    String::from("filter cleared")
                } else {
                    format!("filter set to {:?}", self.filter)
                };
                Ok(Exit::Continue)
            }
            "clear" => {
                self.filter.clear();
                self.selected = 0;
                self.message = String::from("filter cleared");
                Ok(Exit::Continue)
            }
            "refresh" => {
                self.stations = client.get_stations()?;
                self.selected = 0;
                self.message = format!("reloaded {} stations", self.stations.len());
                Ok(Exit::Continue)
            }
            "up" | "k" => {
                let len = self.filtered_stations().len();
                if len > 0 {
                    if self.selected == 0 {
                        self.selected = len - 1;
                    } else {
                        self.selected -= 1;
                    }
                }
                Ok(Exit::Continue)
            }
            "down" | "j" => {
                let len = self.filtered_stations().len();
                if len > 0 {
                    self.selected = (self.selected + 1) % len;
                }
                Ok(Exit::Continue)
            }
            "stop" => {
                player.stop();
                self.message = String::from("stopped playback");
                Ok(Exit::Continue)
            }
            "now" => {
                self.message = self
                    .now_playing(client)
                    .unwrap_or_else(|err| format!("now playing unavailable: {err}"));
                Ok(Exit::Continue)
            }
            "play" | "p" => {
                let selected = if tail.is_empty() {
                    self.filtered_stations().get(self.selected).cloned()
                } else if let Ok(index) = tail.parse::<usize>() {
                    self.filtered_stations()
                        .get(index.saturating_sub(1))
                        .cloned()
                } else {
                    self.filtered_stations()
                        .into_iter()
                        .find(|station| station.title.to_lowercase().contains(&tail.to_lowercase()))
                };

                let station =
                    selected.ok_or_else(|| Error::new("no station matched the request"))?;
                let url = station
                    .stream_url()
                    .ok_or_else(|| Error::new("station has no stream URL"))?;
                player.play(url)?;
                self.message = match self.now_playing_for_station(client, &station) {
                    Ok(Some(track)) if !track.song.is_empty() || !track.artist.is_empty() => {
                        format!(
                            "playing {} - {} / {}",
                            station.title, track.artist, track.song
                        )
                    }
                    Ok(_) => format!("playing {}", station.title),
                    Err(_) => format!("playing {}", station.title),
                };
                Ok(Exit::Continue)
            }
            _ => {
                self.message = format!("unknown command: {head}");
                Ok(Exit::Continue)
            }
        }
    }

    fn filtered_stations(&self) -> Vec<&Station> {
        if self.filter.is_empty() {
            return self.stations.iter().collect();
        }
        let needle = self.filter.to_lowercase();
        self.stations
            .iter()
            .filter(|station| {
                station.title.to_lowercase().contains(&needle)
                    || station.tooltip.to_lowercase().contains(&needle)
                    || station.prefix.to_lowercase().contains(&needle)
            })
            .collect()
    }

    fn now_playing(&self, client: &Client) -> Result<String> {
        let station = self
            .filtered_stations()
            .get(self.selected)
            .copied()
            .ok_or_else(|| Error::new("no station selected"))?;
        match self.now_playing_for_station(client, station)? {
            Some(track) if !track.song.is_empty() || !track.artist.is_empty() => Ok(format!(
                "{}: {} - {} ({})",
                station.title, track.artist, track.song, track.time_formatted
            )),
            _ => Ok(format!("{}: no recent track data", station.title)),
        }
    }

    fn now_playing_for_station(
        &self,
        client: &Client,
        station: &Station,
    ) -> Result<Option<api::Track>> {
        client.get_now_playing(station.id)
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut out = String::new();
    for ch in value.chars().take(max_chars) {
        out.push(ch);
    }
    if value.chars().count() > max_chars {
        out.push('…');
    }
    out
}
