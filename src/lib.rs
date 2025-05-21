pub mod tokens;
pub mod el_regex;
pub mod tokenizer;

pub mod operations;
pub mod parser;

pub mod interpreter;

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_tokenizer () -> ()
    {
        assert_eq!(tokenizer::tokenize_str("Hello (~ wow! ~), 256 'world'!"), vec![tokens::Token::Identifier(String::from("Hello")), tokens::Token::Symbol(tokens::Symbol::Com), tokens::Token::Numeric(256f64), tokens::Token::Text(String::from("world")), tokens::Token::Operator(tokens::Operator::XOr)]);
    }
}
