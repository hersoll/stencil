use std::fmt::Display;

pub struct ContinousSolution {
    content: String,
    /// Denotes whether the math be printed `$inline$` or `$ block $`
    inline: bool,
}

impl ContinousSolution {
    pub(crate) fn inline() -> Self {
        Self {
            content: String::new(),
            inline: true,
        }
    }

    pub(crate) fn block() -> Self {
        Self {
            content: String::new(),
            inline: false,
        }
    }

    pub fn write(&mut self, s: impl Display) -> &mut Self {
        self.content += &format!("{s}");
        self
    }

    pub fn equals(&mut self, s: impl Display) -> &mut Self {
        self.write(" = ");
        self.write(s)
    }

    pub fn newline(&mut self) -> &mut Self {
        self.write(" \\ ")
    }

    /// Assumes you are breaking in an equality and automatically appends `=` as well.
    ///
    /// Use [`newline()`] if you just want a new line
    pub fn linebreak(&mut self) -> &mut Self {
        self.write(" = ");
        self.newline()
    }
}

impl Display for ContinousSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inline {
            true => write!(f, "${}$", self.content),
            false => write!(f, "$ {} $", self.content),
        }
    }
}
