use crate::tokens;
use regex::Regex;

let patterns_en : Vec::<(Regex, Token)> = vec!
    [
        (Regex::new(r"^'(\\.|[^'])*'").unwrap(), tokens::Token::Text(String::new())),

        (Regex::new(r"^`(\\.|[^`])*`").unwrap(), tokens::Token::Identifier(String::new())),

        (Regex::new("^\"[A-Za-z1-9\\-_:]+").unwrap(), tokens::Token::Label(String::new())),

        (Regex::new(r"^let\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::Let)),
        (Regex::new(r"^def(ine)?\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::Def)),
        (Regex::new(r"^as\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::As)),
        (Regex::new(r"^assert\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::Assert)),
        (Regex::new(r"^input\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::Input)),
        (Regex::new(r"^ref(erence)?\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::Ref)),
        (Regex::new(r"^if\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::If)),
        (Regex::new(r"^else\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::Else)),
        (Regex::new(r"^with\b").unwrap(), tokens::Token::Keyword(tokens::Keyword::With)),

        (Regex::new(r"^d+").unwrap(), tokens::Token::Numeric(0)),


        (Regex::new(r"^:=").unwrap(), tokens::Token::Operator(tokens::Operator::Sig)),
        (Regex::new(r"^:").unwrap(), tokens::Token::Operator(tokens::Operator::DDot)),
        (Regex::new(r"^::").unwrap(), tokens::Token::Operator(tokens::Operator::QDot)),
        (Regex::new(r"^->").unwrap(), tokens::Token::Operator(tokens::Operator::RArr)),
        (Regex::new(r"^<-").unwrap(), tokens::Token::Operator(tokens::Operator::LArr)),
        (Regex::new(r"^<->").unwrap(), tokens::Token::Operator(tokens::Operator::DArr)),

        (Regex::new(r"^(+|plus\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Add)),
        (Regex::new(r"^(-|minus\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Sub)),
        (Regex::new(r"^(*|times\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Mul)),
        (Regex::new(r"^(/|divided_by\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Div)),
        (Regex::new(r"^(\^|raised_by\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Exp)),
        (Regex::new(r"^(\^/|root\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Root)),
        (Regex::new(r"^mod\b").unwrap(), tokens::Token::Operator(tokens::Operator::Mod)),

        (Regex::new(r"^(\||or\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Or)),
        (Regex::new(r"^(&|and\b)").unwrap(), tokens::Token::Operator(tokens::Operator::And)),
        (Regex::new(r"^(b\||bor\b)").unwrap(), tokens::Token::Operator(tokens::Operator::BOr)),
        (Regex::new(r"^(b&|band\b)").unwrap(), tokens::Token::Operator(tokens::Operator::BAnd)),
        (Regex::new(r"^(b!=|bxor\b)").unwrap(), tokens::Token::Operator(tokens::Operator::BXOr)),
        (Regex::new(r"^(¬|~|not\b)").unwrap(), tokens::Token::Operator(tokens::Operator::BXOr)),

        (Regex::new(r"^in\b").unwrap(), tokens::Token::Operator(tokens::Operator::In)),
        (Regex::new(r"^(¬|~|n)in\b").unwrap(), tokens::Token::Operator(tokens::Operator::NIn)),

        (Regex::new(r"^(<|is_less_than\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Les)),
        (Regex::new(r"^(>|is_greater_than\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Gre)),
        (Regex::new(r"^(=|eq\b)").unwrap(), tokens::Token::Operator(tokens::Operator::Equ)),
        (Regex::new(r"^(<|l)(=|eq\b)").unwrap(), tokens::Token::Operator(tokens::Operator::LEq)),
        (Regex::new(r"^(>|g)(=|eq\b)").unwrap(), tokens::Token::Operator(tokens::Operator::GEq)),
        (Regex::new(r"^(¬|~|n)(=|eq\b)").unwrap(), tokens::Token::Operator(tokens::Operator::NEq)),


        (Regex::new(r"^(~").unwrap(), tokens::Token::Brace(tokens::Brace::Comment(0))),
        (Regex::new(r"^~)").unwrap(), tokens::Token::Brace(tokens::Brace::Comment(1))),
        (Regex::new(r"^(").unwrap(), tokens::Token::Brace(tokens::Brace::Round(0))),
        (Regex::new(r"^)").unwrap(), tokens::Token::Brace(tokens::Brace::Round(1))),
        (Regex::new(r"^[").unwrap(), tokens::Token::Brace(tokens::Brace::Square(0))),
        (Regex::new(r"^]").unwrap(), tokens::Token::Brace(tokens::Brace::Square(1))),
        (Regex::new(r"^{").unwrap(), tokens::Token::Brace(tokens::Brace::Curly(0))),
        (Regex::new(r"^}").unwrap(), tokens::Token::Brace(tokens::Brace::Curly(1))),

        (Regex::new(r"^[A-Za-z][A-Za-z1-9\-_]*").unwrap(), tokens::Token::Identifier(String::new())),


        (Regex::new(r"^\s+").unwrap(), tokens::Token::Whitespace)
    ];
