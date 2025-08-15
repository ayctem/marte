use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Punctuation
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    Caret,         // ^
    Hash,          // #
    Ampersand,     // &
    Tilde,         // ~
    Pipe,          // |
    DoubleLess,    // <<
    DoubleGreater, // >>
    DoubleSlash,   // //
    DoubleEqual,   // ==
    TildeEqual,    // ~=
    LessEqual,     // <=
    GreaterEqual,  // >=
    Less,          // <
    Greater,       // >
    Equal,         // =
    OpenParen,     // (
    CloseParen,    // )
    OpenBrace,     // {
    CloseBrace,    // }
    OpenBracket,   // [
    CloseBracket,  // ]
    DoubleColon,   // ::
    Semicolon,     // ;
    Colon,         // :
    Comma,         // ,
    Dot,           // .
    DoubleDot,     // ..
    TripleDot,     // ...

    // Literals
    Name(String),
    String(String),
    Numeric(Numeric),

    // Keywords
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Numeric {
    Integer(i64),
    Float(f64),
}
