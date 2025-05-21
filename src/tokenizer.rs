use crate::tokens::*;
use crate::el_regex::*;

pub fn tokenize_str (mut string: &str) -> Vec::<Token>
{
    let mut tokens = Vec::<Token>::new();
    let patterns_la = patterns_la();

    while string.len() != 0
    {
        for pat in patterns_la.iter()
        {
            if let Some(pat_match) = pat.0.find(string)
            {
                if let Token::WhiteSpace | Token::Brace(Brace::Comment) = pat.1
                {
                    string = &string[pat_match.end()..];
                    break
                }

                let result : Token;

                if let Token::Text(_) = pat.1.clone()
                { result = Token::Text(String::from(&string[(pat_match.start() + 1)..(pat_match.end() - 1)])) }
                else if let Token::Identifier(_) = pat.1.clone()
                { result = Token::Identifier(String::from(if &string[pat_match.start()..(pat_match.start() + 1)] == "`" { &string[(pat_match.start() + 1)..(pat_match.end() - 1)] }
                                                          else { pat_match.as_str() })) }
                else if let Token::Numeric(_) = pat.1.clone()
                { result = Token::Numeric(pat_match.as_str().parse::<f64>().unwrap()) }
                else
                { result = pat.1.clone() }

                tokens.push(result);

                string = &string[pat_match.end()..];

                break
            }
        }
    }

    tokens
}
