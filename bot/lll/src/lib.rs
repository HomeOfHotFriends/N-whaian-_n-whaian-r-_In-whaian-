use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode {
    Paired,
    IsolatedX,
    IsolatedY,
    Synthesized,
    Pass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    PreAwake,
    PostAwake,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Coordinate { x: u16, y: u16 },
    Identity { x: u16 },
    State { cartridge: u16, state: u16 },
    Unified { value: u32 },
    Pass,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Coordinate { x, y } => write!(f, "Coordinate({x},{y})"),
            Instruction::Identity { x } => write!(f, "Identity({x})"),
            Instruction::State { cartridge, state } => {
                write!(f, "State(y={state} | x={cartridge})")
            }
            Instruction::Unified { value } => write!(f, "Unified({value})"),
            Instruction::Pass => write!(f, "Pass"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Controller {
    pub enabled: bool,
}

impl Controller {
    pub fn synthesize(&self, pair: Pair, phase: Phase) -> u32 {
        let (hi, lo) = match phase {
            Phase::PreAwake => (pair.x as u32, pair.y as u32),
            Phase::PostAwake => (pair.y as u32, pair.x as u32),
        };
        (hi << 16) | lo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LllMachine {
    pub phase: Phase,
    pub controller: Controller,
}

impl Default for LllMachine {
    fn default() -> Self {
        Self {
            phase: Phase::PreAwake,
            controller: Controller { enabled: true },
        }
    }
}

impl LllMachine {
    pub fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
    }

    pub fn flip_phase(&mut self) {
        self.phase = match self.phase {
            Phase::PreAwake => Phase::PostAwake,
            Phase::PostAwake => Phase::PreAwake,
        };
    }

    pub fn read(&self, pair: Pair, mode_override: Option<ReadMode>) -> Instruction {
        let mode = mode_override.unwrap_or_else(|| {
            if pair.x == 0 && pair.y == 0 {
                ReadMode::Pass
            } else if pair.x == 0 {
                ReadMode::IsolatedY
            } else if pair.y == 0 {
                ReadMode::IsolatedX
            } else {
                ReadMode::Paired
            }
        });

        match mode {
            ReadMode::Pass => Instruction::Pass,
            ReadMode::Paired => Instruction::Coordinate { x: pair.x, y: pair.y },
            ReadMode::IsolatedX => Instruction::Identity { x: pair.x },
            ReadMode::IsolatedY => Instruction::State {
                cartridge: pair.x,
                state: pair.y,
            },
            ReadMode::Synthesized => {
                if self.controller.enabled {
                    Instruction::Unified {
                        value: self.controller.synthesize(pair, self.phase),
                    }
                } else {
                    Instruction::Coordinate { x: pair.x, y: pair.y }
                }
            }
        }
    }
}