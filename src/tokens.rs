#[derive(Debug, Clone, PartialEq)]
pub enum Keyword
{
    Let, Def,
    As,
    Assert, Input,
    Ref,
    If, Else,
    With
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator
{
    Asi,
    Add, Sub, Mul, Div,
    Exp, Root,
    Mod,
    Or, And, XOr, Not,
    BOr, BAnd, BXOr, BNot,
    In, NIn,
    Les, Gre, Equ,
    LEq, GEq, NEq
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol
{
    QDot,
    RArr, LArr, DArr,
    Com, SCol, Col, Per
}

#[derive(Debug, Clone, PartialEq)]
pub enum Brace
{
    Round(bool),
    Square(bool),
    Curly(bool),
    Comment
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token
{
    Keyword(Keyword),
    Text(String),
    Label(String),
    Identifier(String),
    Numeric(f64),
    Operator(Operator),
    Symbol(Symbol),
    Brace(Brace),
    WhiteSpace,
    Unknown
}

