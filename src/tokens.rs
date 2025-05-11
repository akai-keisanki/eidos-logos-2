pub enum Keyword
{
    Let,
    Def,
    As,
    Assert,
    Input,
    Ref,
    If,
    Else,
    With
}

pub enum Operator
{
    Asi,
    DDot,
    QDot,
    RArr,
    LArr,
    DArr,
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Root,
    Mod,
    Or,
    And,
    BOr,
    BAnd,
    BXOr,
    Not,
    In,
    NIn,
    Les,
    Gre,
    Equ,
    LEq,
    GEq,
    NEq
}

pub enum Brace
{
    Round(bool),
    Square(bool),
    Curly(bool),
    Comment(bool)
}

pub enum Token
{
    Keyword(Keyword),
    Text(String),
    Label(String),
    Identifier(String),
    Numeric(f64),
    Operator(Operator),
    Brace(Brace),
    Whitespace,
    Unknown(String)
}

