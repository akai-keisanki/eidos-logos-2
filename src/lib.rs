pub mod operations;
pub mod tokens;

pub mod tokenizer;
pub mod parser;
pub mod interpreter;

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_operation_enum ()
    {
        let operation = operations::Operation::Unary(operations::UnaryOperation::Assertion);

        assert!(matches!(operation, operations::Operation::Unary(_)));

        if let operations::Operation::Unary(unary_operation) = operation
        {
            assert!(matches!(unary_operation, operations::UnaryOperation::Assertion));
        }
    }
}
