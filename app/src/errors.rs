use derive_more::From;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // Static arrays
    EmptyStatic,

    // Registry
    RegistryMutexIsPoisoned,
    NoSuchProblemInRegistry {
        id: String,
    },

    // Problem generation
    InvalidIntRange {
        min: i32,
        max: i32,
    },
    NoValidProblems,
    InvalidDifficulty {
        difficulty: u8,
    },

    // Translations
    InvalidTranslationKey {
        group: String,
        key: String,
        lang: String,
    },

    // Frontend
    WebAPIFailed,
    PDFLoadingFailed,
    PDFDownloadFailed,

    // Externals
    #[from]
    Io(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
