use crate::symbols;

use super::Symbol;
use anyhow::{Context, Result, anyhow};
use rand::{self, seq::IndexedRandom};

struct WeightedSymbol {
    symbol: &'static Symbol,
    weight: u8,
}

impl WeightedSymbol {
    const fn new(symbol: &'static Symbol, weight: u8) -> WeightedSymbol {
        WeightedSymbol { symbol, weight }
    }
}

/// Get a random unknown - the x in 3x + 1 = 10 or (2x + 1) - (x + 2)
pub fn get_unknown() -> Result<&'static Symbol> {
    get_random(&UNKNOWNS)
}

pub fn get_unknown_with_exclusions<T: Into<Vec<&'static str>>>(
    exclusions_primitive: T,
) -> Result<&'static Symbol> {
    let exclusions: Vec<&'static str> = exclusions_primitive.into();
    if exclusions.len() == UNKNOWNS.len() {
        return Err(anyhow!("Too many exclusions when getting unknown"));
    }
    while let Ok(chosen_symbol) = get_random(&UNKNOWNS) {
        if !exclusions.contains(&chosen_symbol.0) {
            return Ok(chosen_symbol);
        }
    }
    Err(anyhow!("Too many exclusions when getting unknown"))
}

pub fn get_two_unknowns() -> Result<(&'static Symbol, &'static Symbol)> {
    let mut rng = rand::rng();
    DOUBLE_UNKNOWNS
        .choose(&mut rng)
        .ok_or(anyhow!("The get_two_unknowns array is somehow empty?"))
        .copied()
}

// pub fn get_three_unknowns() -> Result<(char, char, char)> {
//     let mut rng = rand::rng();
//     [
//         ('a', 'b', 'c'),
//         ('p', 'q', 'r'),
//         ('t', 'u', 'v'),
//         ('x', 'y', 'z'),
//     ]
//     .choose(&mut rng)
//     .ok_or(crate::Error::EmptyStatic)
//     .copied()
// }

/// Get a random function name - the f in f(x).
pub fn get_function_name() -> Result<&'static Symbol> {
    get_random(&FUNCTION_NAMES)
}

/// Get a random **function** variable, i.e. the x in f(x).
pub fn get_variable() -> Result<&'static Symbol> {
    get_random(&VARIABLES)
}

fn get_random(symbols: &[WeightedSymbol]) -> Result<&'static Symbol> {
    let mut rng = rand::rng();
    symbols
        .choose_weighted(&mut rng, |weighted| weighted.weight)
        .context("The symbols array is somehow empty")
        .map(|weighted| weighted.symbol)
}

/// UNKNOWNS are used in equations and expressions - y is fine here
static UNKNOWNS: [WeightedSymbol; 12] = [
    WeightedSymbol::new(symbols::A, 3),
    WeightedSymbol::new(symbols::B, 1),
    WeightedSymbol::new(symbols::C, 1),
    WeightedSymbol::new(symbols::D, 1),
    WeightedSymbol::new(symbols::K, 2),
    WeightedSymbol::new(symbols::P, 1),
    WeightedSymbol::new(symbols::Q, 1),
    WeightedSymbol::new(symbols::R, 1),
    WeightedSymbol::new(symbols::T, 3),
    WeightedSymbol::new(symbols::X, 30),
    WeightedSymbol::new(symbols::Y, 2),
    WeightedSymbol::new(symbols::Z, 1),
];

static DOUBLE_UNKNOWNS: [(&Symbol, &Symbol); 5] = [
    (symbols::A, symbols::B),
    (symbols::J, symbols::K),
    (symbols::M, symbols::N),
    (symbols::P, symbols::Q),
    (symbols::X, symbols::Y),
];

/// Function names are used for the form f(x) - note that y is not included here
/// since it is uncommon in Sweden to explicitly write y(x).
static FUNCTION_NAMES: [WeightedSymbol; 5] = [
    WeightedSymbol::new(symbols::F, 20),
    WeightedSymbol::new(symbols::G, 5),
    WeightedSymbol::new(symbols::H, 4),
    WeightedSymbol::new(symbols::S, 2),
    WeightedSymbol::new(symbols::V, 5),
];

static VARIABLES: [WeightedSymbol; 2] = [
    WeightedSymbol::new(symbols::X, 7),
    WeightedSymbol::new(symbols::T, 1),
];
