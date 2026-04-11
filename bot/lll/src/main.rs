use std::env;

use lll::{LllMachine, Pair, Phase, ReadMode};

fn parse_mode(s: &str) -> Option<ReadMode> {
    match s.to_lowercase().as_str() {
        "paired" => Some(ReadMode::Paired),
        "x" | "isolatedx" => Some(ReadMode::IsolatedX),
        "y" | "isolatedy" => Some(ReadMode::IsolatedY),
        "s" | "synth" | "synthesized" => Some(ReadMode::Synthesized),
        "pass" => Some(ReadMode::Pass),
        _ => None,
    }
}

fn parse_phase(s: &str) -> Option<Phase> {
    match s.to_lowercase().as_str() {
        "pre" | "preawake" => Some(Phase::PreAwake),
        "post" | "postawake" => Some(Phase::PostAwake),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: lll <x> <y> [mode] [phase]");
        eprintln!("modes: paired | x | y | synth | pass");
        eprintln!("phase: pre | post");
        std::process::exit(1);
    }

    let x: u16 = args[1].parse().expect("x must be a number");
    let y: u16 = args[2].parse().expect("y must be a number");

    let mode = args.get(3).and_then(|s| parse_mode(s));
    let phase = args.get(4).and_then(|s| parse_phase(s));

    let mut lll = LllMachine::default();
    if let Some(p) = phase {
        lll.set_phase(p);
    }

    let instruction = lll.read(Pair { x, y }, mode);
    println!("{}", instruction);
}