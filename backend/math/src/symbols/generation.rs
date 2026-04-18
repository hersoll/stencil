use super::Symbol;
use anyhow::{Context, Result, anyhow};
use rand::{self, seq::IndexedRandom};

struct WeightedSymbol {
    symbol: Symbol,
    weight: u8,
}

impl WeightedSymbol {
    const fn new(symbol: Symbol, weight: u8) -> WeightedSymbol {
        WeightedSymbol { symbol, weight }
    }
}

/// Get a random unknown - the x in 3x + 1 = 10 or (2x + 1) - (x + 2)
pub fn get_unknown() -> Result<Symbol> {
    get_random(&UNKNOWNS)
}

pub fn get_unknown_with_exclusions<T: Into<Vec<&'static str>>>(
    exclusions_primitive: T,
) -> Result<Symbol> {
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

pub fn get_two_unknowns() -> Result<(Symbol, Symbol)> {
    let mut rng = rand::rng();
    [
        (Symbol("a"), Symbol("b")),
        (Symbol("j"), Symbol("k")),
        (Symbol("m"), Symbol("n")),
        (Symbol("p"), Symbol("q")),
        (Symbol("t"), Symbol("u")),
        (Symbol("x"), Symbol("y")),
    ]
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
pub fn get_function_name() -> Result<Symbol> {
    get_random(&FUNCTION_NAMES)
}

/// Get a random **function** variable, i.e. the x in f(x).
pub fn get_variable() -> Result<Symbol> {
    get_random(&VARIABLES)
}

fn get_random(symbols: &[WeightedSymbol]) -> Result<Symbol> {
    let mut rng = rand::rng();
    symbols
        .choose_weighted(&mut rng, |weighted| weighted.weight)
        .context("The symbols array is somehow empty")
        .map(|weighted| weighted.symbol)
}

/// UNKNOWNS are used in equations and expressions - y is fine here
static UNKNOWNS: [WeightedSymbol; 12] = [
    WeightedSymbol::new(Symbol("a"), 3),
    WeightedSymbol::new(Symbol("b"), 1),
    WeightedSymbol::new(Symbol("c"), 1),
    WeightedSymbol::new(Symbol("d"), 1),
    WeightedSymbol::new(Symbol("k"), 2),
    WeightedSymbol::new(Symbol("p"), 1),
    WeightedSymbol::new(Symbol("q"), 1),
    WeightedSymbol::new(Symbol("r"), 1),
    WeightedSymbol::new(Symbol("t"), 3),
    WeightedSymbol::new(Symbol("x"), 30),
    WeightedSymbol::new(Symbol("y"), 2),
    WeightedSymbol::new(Symbol("z"), 1),
];

/// Function names are used for the form f(x) - note that y is not included here
/// since it is uncommon in Sweden to explicitly write y(x).
static FUNCTION_NAMES: [WeightedSymbol; 5] = [
    WeightedSymbol::new(Symbol("f"), 20),
    WeightedSymbol::new(Symbol("g"), 5),
    WeightedSymbol::new(Symbol("h"), 4),
    WeightedSymbol::new(Symbol("s"), 2),
    WeightedSymbol::new(Symbol("v"), 5),
];

static VARIABLES: [WeightedSymbol; 2] = [
    WeightedSymbol::new(Symbol("x"), 7),
    WeightedSymbol::new(Symbol("t"), 1),
];
