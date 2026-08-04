use std::fmt::Display;

enum DisplayType {
    Inline,
    /// Block alone with no text above it. Will take up as little space as possible to look good.
    Block,
    /// Block that has normal explaining text before or after. Will span the entire width.
    BlockWithText,
}

pub struct ContinuousSolution {
    content: String,
    /// Denotes whether the math be printed `$inline$` or `$ block $`
    display: DisplayType,
}

impl ContinuousSolution {
    pub(crate) fn inline() -> Self {
        Self {
            content: String::new(),
            display: DisplayType::Inline,
        }
    }

    pub(crate) fn block() -> Self {
        Self {
            content: String::new(),
            display: DisplayType::Block,
        }
    }

    pub(crate) fn block_with_text() -> Self {
        Self {
            content: String::new(),
            display: DisplayType::BlockWithText,
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

    pub fn space(&mut self) -> &mut Self {
        self.write(" quad ")
    }

    pub fn wide_space(&mut self) -> &mut Self {
        self.write(" wide ")
    }
}

impl Display for ContinuousSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DisplayType::*;
        match self.display {
            Inline => write!(f, "${}$", self.content),
            Block => write!(
                f,
                "#context {{ let eq = $ {} $ 
                let w = measure(eq).width 
                box(width: w, eq) 
                }} ",
                self.content
            ),
            BlockWithText => write!(f, "$ {} $", self.content),
        }
    }
}
