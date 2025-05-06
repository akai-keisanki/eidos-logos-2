pub enum Keyword
{
    Let,
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
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
    Or,
    And,
    XOr,
    BOr,
    BAnd,
    BXOr,
    In
}

pub enum Token
{
    Keyword(Keyword),
    Identifier(String),
    Numeric(f64),
    Operator(Operator),
    Unknown(String)
}
