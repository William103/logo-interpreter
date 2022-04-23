use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(u64),

    #[regex("[a-zA-Z][a-zA-Z_-]*", |lex| Some(lex.slice().into()))]
    Command(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
