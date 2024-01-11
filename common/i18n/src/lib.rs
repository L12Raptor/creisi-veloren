use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// The type to represent generic localization request, to be sent from server
/// to client and then localized (or internationalized) there.
// TODO: This could be generalised to *any* in-game text, not just chat messages (hence it not being
// called `ChatContent`). A few examples:
//
// - Signposts, both those appearing as overhead messages and those displayed 'in-world' on a shop
//   sign
// - UI elements
// - In-game notes/books (we could add a variant that allows structuring complex, novel textual
//   information as a syntax tree or some other intermediate format that can be localised by the
//   client)
// TODO: We probably want to have this type be able to represent similar things to
// `fluent::FluentValue`, such as numeric values, so that they can be properly localised in whatever
// manner is required.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Content {
    /// The content is a plaintext string that should be shown to the user
    /// verbatim.
    Plain(String),
    /// The content is a localizable message with the given arguments.
    // TODO: reduce usages of random i18n as much as possible
    //
    // It's ok to have random messages, just not at i18n step.
    // Look for issue on `get_vartion` at Gitlab for more.
    Localized {
        /// i18n key
        key: String,
        /// Pseudorandom seed value that allows frontends to select a
        /// deterministic (but pseudorandom) localised output
        #[serde(default = "random_seed")]
        seed: u16,
        /// i18n arguments
        #[serde(default)]
        args: HashMap<String, LocalizationArg>,
    },
}

// TODO: Remove impl and make use of `Plain(...)` explicit (to discourage it)
impl From<String> for Content {
    fn from(text: String) -> Self { Self::Plain(text) }
}

// TODO: Remove impl and make use of `Plain(...)` explicit (to discourage it)
impl<'a> From<&'a str> for Content {
    fn from(text: &'a str) -> Self { Self::Plain(text.to_string()) }
}

/// A localisation argument for localised content (see [`Content::Localized`]).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocalizationArg {
    /// The localisation argument is itself a section of content.
    ///
    /// Note that this allows [`Content`] to recursively refer to itself. It may
    /// be tempting to decide to parameterise everything, having dialogue
    /// generated with a compact tree. "It's simpler!", you might say. False.
    /// Over-parameterisation is an anti-pattern that hurts translators. Where
    /// possible, prefer fewer levels of nesting unless doing so would result
    /// in an intractably larger number of combinations. See [here] for the
    /// guidance provided by the docs for `fluent`, the localisation library
    /// used by clients.
    ///
    /// [here]: https://github.com/projectfluent/fluent/wiki/Good-Practices-for-Developers#prefer-wet-over-dry
    Content(Content),
    /// The localisation argument is a natural number
    Nat(u64),
}

impl From<Content> for LocalizationArg {
    fn from(content: Content) -> Self { Self::Content(content) }
}

// TODO: Remove impl and make use of `Content(Plain(...))` explicit (to
// discourage it)
impl From<String> for LocalizationArg {
    fn from(text: String) -> Self { Self::Content(Content::Plain(text)) }
}

// TODO: Remove impl and make use of `Content(Plain(...))` explicit (to
// discourage it)
impl<'a> From<&'a str> for LocalizationArg {
    fn from(text: &'a str) -> Self { Self::Content(Content::Plain(text.to_string())) }
}

// TODO: Remove impl and make use of `Content(Plain(...))` explicit (to
// discourage it)
impl From<u64> for LocalizationArg {
    fn from(n: u64) -> Self { Self::Nat(n) }
}

fn random_seed() -> u16 { rand::random() }

impl Content {
    pub fn localized(key: impl ToString) -> Self {
        Self::Localized {
            key: key.to_string(),
            seed: random_seed(),
            args: HashMap::default(),
        }
    }

    pub fn localized_with_args<'a, A: Into<LocalizationArg>>(
        key: impl ToString,
        args: impl IntoIterator<Item = (&'a str, A)>,
    ) -> Self {
        Self::Localized {
            key: key.to_string(),
            seed: rand::random(),
            args: args
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        }
    }

    pub fn as_plain(&self) -> Option<&str> {
        match self {
            Self::Plain(text) => Some(text.as_str()),
            Self::Localized { .. } => None,
        }
    }
}
