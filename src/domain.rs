use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Room {
    LivingRoom,
    BedRoom,
    Kitchen,
    Bath,
}

impl Room {
    pub const ALL: [Room; 4] = [Room::LivingRoom, Room::BedRoom, Room::Kitchen, Room::Bath];

    pub fn as_str(self) -> &'static str {
        match self {
            Room::LivingRoom => "Wohnzimmer",
            Room::BedRoom => "Schlafzimmer",
            Room::Kitchen => "Küche",
            Room::Bath => "Bad",
        }
    }

    pub fn parse(input: &str) -> Result<Self> {
        match input.trim().to_lowercase().as_str() {
            "living" | "livingroom" | "wohnzimmer" => Ok(Room::LivingRoom),
            "bedroom" | "schlafzimmer" | "bed" => Ok(Room::BedRoom),
            "kitchen" | "küche" | "kueche" => Ok(Room::Kitchen),
            "bath" | "bad" | "bathroom" => Ok(Room::Bath),
            other => Err(anyhow!("Unbekannter Raum: {other}")),
        }
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RoomSelection {
    One(Room),
    All,
}

impl RoomSelection {
    pub fn parse(input: &str) -> Result<Self> {
        let trimmed = input.trim().to_lowercase();
        if trimmed == "all" || trimmed == "alle" {
            return Ok(RoomSelection::All);
        }
        Ok(RoomSelection::One(Room::parse(trimmed.as_str())?))
    }

    pub fn iter(self) -> Box<dyn Iterator<Item = Room>> {
        match self {
            RoomSelection::One(r) => Box::new(std::iter::once(r)),
            RoomSelection::All => Box::new(Room::ALL.into_iter()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OnOff {
    On,
    Off,
}

impl OnOff {
    pub fn as_bool(self) -> bool {
        matches!(self, OnOff::On)
    }

    pub fn parse(input: &str) -> Result<Self> {
        match input.trim().to_lowercase().as_str() {
            "on" | "an" => Ok(OnOff::On),
            "off" | "aus" => Ok(OnOff::Off),
            other => Err(anyhow!("Ungültiger Zustand: {other}. Erlaubt: on/off")),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TemperatureC(pub f32);

impl TemperatureC {
    pub fn new(value: f32) -> Option<Self> {
        if (5.0..=35.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

// Einführung von Szenen (Scene-Funktion).
// Eine Szene fasst mehrere Aktionen zusammen (z.B. Night, Away, Morning),
// um typische SmartHome-Zustände schnell zu aktivieren.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Scene {
    Night,
    Away,
    Morning,
}

impl Scene {
    pub fn parse(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "night" | "nacht" => Some(Scene::Night),
            "away" | "abwesend" => Some(Scene::Away),
            "morning" | "morgen" => Some(Scene::Morning),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    HeatingSet {
        where_: RoomSelection,
        target: TemperatureC,
    },
    HeatingEnabled {
        where_: RoomSelection,
        enabled: bool,
    },
    LightsSet {
        where_: RoomSelection,
        state: OnOff,
    },
    LightsToggle {
        where_: RoomSelection,
    },
    SecurityLockAll,
    SecurityUnlockAll,
    // Neue Funktion: Szenensteuerung
    Scene(Scene),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HeatingState {
    enabled: bool,
    target: TemperatureC,
}

impl Default for HeatingState {
    fn default() -> Self {
        Self {
            enabled: false,
            target: TemperatureC(20.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LightState {
    on: bool,
}

impl Default for LightState {
    fn default() -> Self {
        Self { on: false }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HeatingSystem {
    per_room: HashMap<Room, HeatingState>,
}

impl HeatingSystem {
    fn new_default() -> Self {
        let per_room = Room::ALL
            .into_iter()
            .map(|r| (r, HeatingState::default()))
            .collect();
        Self { per_room }
    }

    fn set_target(&mut self, where_: RoomSelection, target: TemperatureC) {
        for room in where_.iter() {
            if let Some(state) = self.per_room.get_mut(&room) {
                state.enabled = true;
                state.target = target;
            }
        }
    }

    fn set_enabled(&mut self, where_: RoomSelection, enabled: bool) {
        for room in where_.iter() {
            if let Some(state) = self.per_room.get_mut(&room) {
                state.enabled = enabled;
            }
        }
    }

    fn status_lines(&self) -> Vec<String> {
        Room::ALL
            .into_iter()
            .filter_map(|r| self.per_room.get(&r).map(|s| (r, s)))
            .map(|(room, state)| {
                let on_off = if state.enabled { "AN" } else { "AUS" };
                format!(
                    "Heizung {}: {} (Ziel: {:.1}°C)",
                    room, on_off, state.target.0
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LightingSystem {
    per_room: HashMap<Room, LightState>,
}

impl LightingSystem {
    fn new_default() -> Self {
        let per_room = Room::ALL
            .into_iter()
            .map(|r| (r, LightState::default()))
            .collect();
        Self { per_room }
    }

    fn set(&mut self, where_: RoomSelection, state: OnOff) {
        let on = state.as_bool();
        for room in where_.iter() {
            if let Some(light) = self.per_room.get_mut(&room) {
                light.on = on;
            }
        }
    }

    fn toggle(&mut self, where_: RoomSelection) {
        for room in where_.iter() {
            if let Some(light) = self.per_room.get_mut(&room) {
                light.on = !light.on;
            }
        }
    }

    fn status_lines(&self) -> Vec<String> {
        Room::ALL
            .into_iter()
            .filter_map(|r| self.per_room.get(&r).map(|s| (r, s)))
            .map(|(room, state)| {
                let on_off = if state.on { "AN" } else { "AUS" };
                format!("Licht {}: {}", room, on_off)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecuritySystem {
    locked: bool,
}

impl SecuritySystem {
    fn new_default() -> Self {
        Self { locked: false }
    }

    fn lock_all(&mut self) {
        self.locked = true;
    }

    fn unlock_all(&mut self) {
        self.locked = false;
    }

    fn status_line(&self) -> String {
        if self.locked {
            "Sicherheit: Haus ist VERRIEGELT".to_string()
        } else {
            "Sicherheit: Haus ist ENTRIEGELT".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartHome {
    heating: HeatingSystem,
    lighting: LightingSystem,
    security: SecuritySystem,
}

impl SmartHome {
    pub fn new_default() -> Self {
        Self {
            heating: HeatingSystem::new_default(),
            lighting: LightingSystem::new_default(),
            security: SecuritySystem::new_default(),
        }
    }

// Implementierung der Szenenlogik.
// Jede Szene setzt mehrere Systeme gleichzeitig (Licht, Heizung, Sicherheit).
// Beispiel: "Night" -> Licht aus, Heizung an, Haus verriegeln.
pub fn apply(&mut self, cmd: Command) -> bool {
    match cmd {
        Command::HeatingSet { where_, target } => self.heating.set_target(where_, target),
        Command::HeatingEnabled { where_, enabled } => self.heating.set_enabled(where_, enabled),
        Command::LightsSet { where_, state } => self.lighting.set(where_, state),
        Command::LightsToggle { where_ } => self.lighting.toggle(where_),
        Command::SecurityLockAll => self.security.lock_all(),
        Command::SecurityUnlockAll => self.security.unlock_all(),

        Command::Scene(scene) => {
            match scene {
                Scene::Night => {
                    self.lighting.set(RoomSelection::All, OnOff::Off);
                    self.heating.set_enabled(RoomSelection::All, true);
                    self.security.lock_all();
                }
                Scene::Away => {
                    self.lighting.set(RoomSelection::All, OnOff::Off);
                    self.heating.set_enabled(RoomSelection::All, false);
                    self.security.lock_all();
                }
                Scene::Morning => {
                    self.lighting.set(RoomSelection::All, OnOff::On);
                    self.heating.set_enabled(RoomSelection::All, true);
                    self.security.unlock_all();
                }
            }
        }
    }

    true
}

    pub fn render_status(&self) -> String {
        let mut lines = Vec::new();
        lines.push("=== SmartHome Status ===".to_string());
        lines.push(self.security.status_line());
        lines.push("---".to_string());
        lines.extend(self.heating.status_lines());
        lines.push("---".to_string());
        lines.extend(self.lighting.status_lines());
        lines.join("\n")
    }
}

// Einfache Unit-Tests für zentrale Funktionen (Temperaturvalidierung, Szenen, Licht).
// Dient zur Sicherstellung der korrekten Logik.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperature_validation() {
        assert!(TemperatureC::new(4.0).is_none());
        assert!(TemperatureC::new(20.0).is_some());
        assert!(TemperatureC::new(35.0).is_some());
        assert!(TemperatureC::new(40.0).is_none());
    }

    #[test]
    fn scene_away_sets_expected_state() {
        let mut home = SmartHome::new_default();
        let changed = home.apply(Command::Scene(Scene::Away));
        assert!(changed);

        assert!(home.security.locked);

        // All lights should be off
        for room in Room::ALL {
            let state = home.lighting.per_room.get(&room).unwrap();
            assert!(!state.on);
        }

        // All heating should be disabled
        for room in Room::ALL {
            let state = home.heating.per_room.get(&room).unwrap();
            assert!(!state.enabled);
        }
    }

    #[test]
    fn lights_toggle_changes_state() {
        let mut home = SmartHome::new_default();
        home.apply(Command::LightsToggle {
            where_: RoomSelection::One(Room::Kitchen),
        });

        let kitchen = home.lighting.per_room.get(&Room::Kitchen).unwrap();
        assert!(kitchen.on);
    }
}