use math::{Number, Polynomial};

pub trait Evaluable {
    fn evaluate<T: Into<Number> + Clone>(&self, replacements: &[(char, T)]) -> Number;
}

impl Evaluable for Polynomial {
    fn evaluate<T: Into<Number> + Clone>(&self, replacements: &[(char, T)]) -> Number {
        let replacement_numbers: Vec<(char, Number)> = replacements
            .iter()
            .map(|(c, t)| (*c, t.clone().into()))
            .collect();
        let mut variables: Vec<char> = replacement_numbers.iter().map(|&(c, _)| c).collect();
        variables.sort();
        assert_eq!(
            variables,
            self.get_variables(),
            "Called evaluate() with a mismatch of variables:"
        );

        let mut result: Number = 0.into();
        self.terms.iter().for_each(|term| {
            result += term.evaluate(&replacement_numbers);
        });
        result
    }
}
