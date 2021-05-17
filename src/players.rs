use crate::enums;
use anyhow::{anyhow, Result};
use peppi::frame::Post;
use peppi::game::Game;
use peppi::ubjson::Object;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub code: String,
    pub tag: String,
    pub port: usize,
    pub stocks: u8,
    pub character: Option<String>,
    pub team: Option<String>,
    pub damage: f32,
    pub winner: Cell<bool>,
}

/// Get the game state on the last frame.
fn last_frame(game: &Game, port: usize) -> Option<&Post> {
    return game
        .ports
        .get(port)
        .and_then(|p| p.as_ref())
        .and_then(|p| p.leader.post.last());
}

fn tag_map(game: &Game, port: usize) -> Option<&HashMap<String, Object>> {
    let port = port.to_string();
    let players = game.metadata.json.get("players")?;
    if let Object::Map(hm) = players {
        if let Object::Map(n) = hm.get(&port)? {
            if let Object::Map(netplay) = n.get("names")? {
                return Some(netplay);
            }
        }
    }

    None
}

fn get_tag<'a>(key: &'a str, tags: &'a HashMap<String, Object>) -> Option<&'a String> {
    match tags.get(key)? {
        Object::Str(s) => Some(s),
        _ => None,
    }
}

fn team(game: &Game, port: usize) -> Option<String> {
    game.start.players.get(port).and_then(|p| {
        p.as_ref()
            .and_then(|p| p.team.as_ref())
            .and_then(|t| enums::team(t.color))
    })
}

fn character(game: &Game, port: usize) -> Option<String> {
    game.start
        .players
        .get(port)
        .and_then(|p| p.as_ref().and_then(|p| enums::character(p.character)))
}

/// Gets the state of all players on the last frame of the game.
pub fn player_states(game: &Game) -> Vec<Player> {
    let mut players = Vec::new();

    for port in 0..4 {
        if let Some(post) = last_frame(&game, port) {
            let tags = tag_map(&game, port);

            if tags.is_none() {
                continue;
            }

            let tags = tags.unwrap();
            let code = get_tag("code", tags);
            let tag = get_tag("netplay", tags);

            if code.is_none() || tag.is_none() {
                continue;
            }

            players.push(Player {
                port,
                stocks: post.stocks,
                damage: post.damage,
                code: code.unwrap().to_string(),
                tag: tag.unwrap().to_string(),
                team: team(&game, port),
                character: character(&game, port),
                winner: Cell::new(false),
            });
        }
    }

    players
}

/// Checks if the living players are all on the same team.
fn on_same_team(living: &Vec<Player>) -> bool {
    let winner = living.get(0);
    if let Some(winner) = winner {
        living
            .iter()
            .all(|player| match (&player.team, &winner.team) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            })
    } else {
        false
    }
}

#[derive(Debug)]
struct Tiebreak {
    stocks: u8,
    damage: f32,
    indices: Vec<usize>,
    color: String,
}

impl Tiebreak {
    fn doubles(living: &Vec<Player>) -> String {
        let mut teams: Vec<Tiebreak> = Vec::new();
        for (i, p) in living.iter().enumerate() {
            let color = p.team.as_ref().unwrap();

            let tb = teams.iter_mut().find(|t| t.color == *color);
            match tb {
                Some(mut tb) => {
                    tb.stocks += p.stocks;
                    tb.damage += p.damage;
                    tb.indices.push(i);
                }
                None => teams.push(Tiebreak {
                    stocks: p.stocks,
                    damage: p.damage,
                    color: p.team.as_ref().unwrap().to_string(),
                    indices: vec![i],
                }),
            }
        }

        teams.sort_by(|a, b| {
            if a.stocks > b.stocks {
                return Ordering::Less;
            } else if living[0].stocks < living[1].stocks {
                return Ordering::Greater;
            } else {
                if living[0].damage < living[1].damage {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
        });

        return teams[0].color.clone();
    }

    fn singles(living: &Vec<Player>) {
        let port;

        if living[0].stocks > living[1].stocks {
            port = 0;
        } else if living[0].stocks < living[1].stocks {
            port = 1;
        } else {
            if living[0].damage > living[1].damage {
                port = 1;
            } else {
                port = 0;
            }
        }

        living[port].winner.set(true);
    }
}

/// Make everyone on `team_color` a winner.
fn set_team_winners(team_color: &str, players: &Vec<Player>) {
    players
        .iter()
        .filter(|p| match &p.team {
            Some(t) => t == team_color,
            _ => false,
        })
        .for_each(|t| t.winner.set(true));
}

/** Steps for determining winners.
 *
 * 1. Remove players with 0 stocks.
 * 2. If 1 player:
 *    a. If team, find their teammate.
 *    b. else player is only winner.
 * 3. If 2 or more players:
 *    a. if same team (2 players), return both of them.
 *    b. else compare stocks and damage.
 */
pub fn determine_winners(players: &Vec<Player>, is_teams: bool) -> Result<()> {
    let living: Vec<Player> = players
        .into_iter()
        .filter(|p| p.stocks > 0)
        .cloned()
        .collect();

    if living.len() == 0 {
        return Err(anyhow!("invalid player state"));
    }

    if living.len() == 1 || (living.len() > 2 && on_same_team(players)) {
        living[0].winner.set(true);

        // Check for teammates.
        if let Some(team) = &living[0].team {
            set_team_winners(team, players);
        }

        return Ok(());
    }

    if is_teams {
        let color = Tiebreak::doubles(&living);
        set_team_winners(&color, players);
    } else {
        Tiebreak::singles(&living);
    }

    Ok(())
}
