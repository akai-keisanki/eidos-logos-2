pub enum UnaryOperation
{
    Assertion,
    Input,
    Referenciation
}

pub enum BinaryOperation
{
    Addition,
    Multiplication,
    Subtraction,
    Division,
    Exponentiation,
    Mod,
    Root,
    Declaration
}

pub enum TernaryOperation
{
    Condition,
    Definition
}

pub enum Operation
{
    Unary(UnaryOperation),
    Binary(BinaryOperation),
    Ternary(TernaryOperation)
}
