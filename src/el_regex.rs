use crate::tokens::*;
use regex::Regex;

pub fn patterns_la () -> Vec::<(Regex, Token)>
{
    vec!
        [
            (Regex::new(r"^'(\\.|[^'])*'").unwrap(), Token::Text(String::new())),

            (Regex::new(r"^`(\\.|[^`])*`").unwrap(), Token::Identifier(String::new())),

            (Regex::new("^\"[A-Za-z1-9\\-_:]+").unwrap(), Token::Label(String::new())),

            (Regex::new(r"^dec(lara)?\b").unwrap(), Token::Keyword(Keyword::Let)),
            (Regex::new(r"^def(ini)?\b").unwrap(), Token::Keyword(Keyword::Def)),
            (Regex::new(r"^ut\b").unwrap(), Token::Keyword(Keyword::As)),
            (Regex::new(r"^assere\b").unwrap(), Token::Keyword(Keyword::Assert)),
            (Regex::new(r"^accipe\b").unwrap(), Token::Keyword(Keyword::Input)),
            (Regex::new(r"^ref(eri(_ad)?)?\b").unwrap(), Token::Keyword(Keyword::Ref)),
            (Regex::new(r"^si\b").unwrap(), Token::Keyword(Keyword::If)),
            (Regex::new(r"^aliter\b").unwrap(), Token::Keyword(Keyword::Else)),
            (Regex::new(r"^pro\b").unwrap(), Token::Keyword(Keyword::With)),

            (Regex::new(r"^(\.\d+|\d+(\.\d+)?)").unwrap(), Token::Numeric(0f64)),


            (Regex::new(r"^:=").unwrap(), Token::Operator(Operator::Asi)),

            (Regex::new(r"^::").unwrap(), Token::Symbol(Symbol::QDot)),
            (Regex::new(r"^->").unwrap(), Token::Symbol(Symbol::RArr)),
            (Regex::new(r"^<-").unwrap(), Token::Symbol(Symbol::LArr)),
            (Regex::new(r"^<->").unwrap(), Token::Symbol(Symbol::DArr)),
            (Regex::new(r"^,").unwrap(), Token::Symbol(Symbol::Com)),
            (Regex::new(r"^;").unwrap(), Token::Symbol(Symbol::SCol)),
            (Regex::new(r"^:").unwrap(), Token::Symbol(Symbol::Col)),
            (Regex::new(r"^\.").unwrap(), Token::Symbol(Symbol::Per)),

            (Regex::new(r"^(\+|plus\b)").unwrap(), Token::Operator(Operator::Add)),
            (Regex::new(r"^(-|minus\b)").unwrap(), Token::Operator(Operator::Sub)),
            (Regex::new(r"^(\*|multiplicatum_per\b)").unwrap(), Token::Operator(Operator::Mul)),
            (Regex::new(r"^(/|divisum_per\b)").unwrap(), Token::Operator(Operator::Div)),
            (Regex::new(r"^(\^|ad_potestatem\b)").unwrap(), Token::Operator(Operator::Exp)),
            (Regex::new(r"^(\^/|radix\b)").unwrap(), Token::Operator(Operator::Root)),
            (Regex::new(r"^mod(ulo)?\b").unwrap(), Token::Operator(Operator::Mod)),

            (Regex::new(r"^(\||vel\b)").unwrap(), Token::Operator(Operator::Or)),
            (Regex::new(r"^(&|et\b)").unwrap(), Token::Operator(Operator::And)),
            (Regex::new(r"^(!(=)?|aut\b)").unwrap(), Token::Operator(Operator::XOr)),
            (Regex::new(r"^(¬|~|non\b)").unwrap(), Token::Operator(Operator::Not)),
            (Regex::new(r"^b(\||vel\b)").unwrap(), Token::Operator(Operator::BOr)),
            (Regex::new(r"^b(&|et\b)").unwrap(), Token::Operator(Operator::BAnd)),
            (Regex::new(r"^b(!(=)?|aut\b)").unwrap(), Token::Operator(Operator::BXOr)),
            (Regex::new(r"^b(¬|~|non\b)").unwrap(), Token::Operator(Operator::BNot)),

            (Regex::new(r"^in\b").unwrap(), Token::Operator(Operator::In)),
            (Regex::new(r"^(¬|~|n)in\b").unwrap(), Token::Operator(Operator::NIn)),

            (Regex::new(r"^(<|est_minor_quam\b)").unwrap(), Token::Operator(Operator::Les)),
            (Regex::new(r"^(>|est_maius_quam\b)").unwrap(), Token::Operator(Operator::Gre)),
            (Regex::new(r"^(=|aeq(uat)?\b)").unwrap(), Token::Operator(Operator::Equ)),
            (Regex::new(r"^(<|min(or_)?)(=|aeq(uat)?\b)").unwrap(), Token::Operator(Operator::LEq)),
            (Regex::new(r"^(>|mai(us_)?)(=|aeq(uat)?\b)").unwrap(), Token::Operator(Operator::GEq)),
            (Regex::new(r"^(¬|~|n(on_)?)(=|aeq(uat)?\b)").unwrap(), Token::Operator(Operator::NEq)),


            (Regex::new(r"^\(~(\\.|[^~\\])*~\)").unwrap(), Token::Brace(Brace::Comment)),
            (Regex::new(r"^\(").unwrap(), Token::Brace(Brace::Round(false))),
            (Regex::new(r"^\)").unwrap(), Token::Brace(Brace::Round(true))),
            (Regex::new(r"^\[").unwrap(), Token::Brace(Brace::Square(false))),
            (Regex::new(r"^\]").unwrap(), Token::Brace(Brace::Square(true))),
            (Regex::new(r"^\{").unwrap(), Token::Brace(Brace::Curly(false))),
            (Regex::new(r"^\}").unwrap(), Token::Brace(Brace::Curly(true))),

            (Regex::new(r"^[A-Za-z][A-Za-z1-9\-_]*").unwrap(), Token::Identifier(String::new())),


            (Regex::new(r"^\s+").unwrap(), Token::WhiteSpace),
            (Regex::new(r".").unwrap(), Token::Unknown)
        ]
}

pub fn patterns_en () -> Vec::<(Regex, Token)>
{
    vec!
        [
            (Regex::new(r"^'(\\.|[^'])*'").unwrap(), Token::Text(String::new())),

            (Regex::new(r"^`(\\.|[^`])*`").unwrap(), Token::Identifier(String::new())),

            (Regex::new("^\"[A-Za-z1-9\\-_:]+").unwrap(), Token::Label(String::new())),

            (Regex::new(r"^let\b").unwrap(), Token::Keyword(Keyword::Let)),
            (Regex::new(r"^def(ine)?\b").unwrap(), Token::Keyword(Keyword::Def)),
            (Regex::new(r"^as\b").unwrap(), Token::Keyword(Keyword::As)),
            (Regex::new(r"^assert\b").unwrap(), Token::Keyword(Keyword::Assert)),
            (Regex::new(r"^input\b").unwrap(), Token::Keyword(Keyword::Input)),
            (Regex::new(r"^ref(er(_to)?)?\b").unwrap(), Token::Keyword(Keyword::Ref)),
            (Regex::new(r"^if\b").unwrap(), Token::Keyword(Keyword::If)),
            (Regex::new(r"^else\b").unwrap(), Token::Keyword(Keyword::Else)),
            (Regex::new(r"^with\b").unwrap(), Token::Keyword(Keyword::With)),

            (Regex::new(r"^(\.\d+|\d+(\.\d+)?)").unwrap(), Token::Numeric(0f64)),


            (Regex::new(r"^:=").unwrap(), Token::Operator(Operator::Asi)),

            (Regex::new(r"^::").unwrap(), Token::Symbol(Symbol::QDot)),
            (Regex::new(r"^->").unwrap(), Token::Symbol(Symbol::RArr)),
            (Regex::new(r"^<-").unwrap(), Token::Symbol(Symbol::LArr)),
            (Regex::new(r"^<->").unwrap(), Token::Symbol(Symbol::DArr)),
            (Regex::new(r"^,").unwrap(), Token::Symbol(Symbol::Com)),
            (Regex::new(r"^;").unwrap(), Token::Symbol(Symbol::SCol)),
            (Regex::new(r"^:").unwrap(), Token::Symbol(Symbol::Col)),
            (Regex::new(r"^\.").unwrap(), Token::Symbol(Symbol::Per)),

            (Regex::new(r"^(\+|plus\b)").unwrap(), Token::Operator(Operator::Add)),
            (Regex::new(r"^(-|minus\b)").unwrap(), Token::Operator(Operator::Sub)),
            (Regex::new(r"^(\*|times\b)").unwrap(), Token::Operator(Operator::Mul)),
            (Regex::new(r"^(/|divided_by\b)").unwrap(), Token::Operator(Operator::Div)),
            (Regex::new(r"^(\^|raised_by\b)").unwrap(), Token::Operator(Operator::Exp)),
            (Regex::new(r"^(\^/|root\b)").unwrap(), Token::Operator(Operator::Root)),
            (Regex::new(r"^mod(ulo)?\b").unwrap(), Token::Operator(Operator::Mod)),

            (Regex::new(r"^(\||or\b)").unwrap(), Token::Operator(Operator::Or)),
            (Regex::new(r"^(&|and\b)").unwrap(), Token::Operator(Operator::And)),
            (Regex::new(r"^(!(=)?|and\b)").unwrap(), Token::Operator(Operator::XOr)),
            (Regex::new(r"^(¬|~|not\b)").unwrap(), Token::Operator(Operator::Not)),
            (Regex::new(r"^b(\||or\b)").unwrap(), Token::Operator(Operator::BOr)),
            (Regex::new(r"^b(&|and\b)").unwrap(), Token::Operator(Operator::BAnd)),
            (Regex::new(r"^b(!(=)?|xor\b)").unwrap(), Token::Operator(Operator::BXOr)),
            (Regex::new(r"^b(¬|~|not\b)").unwrap(), Token::Operator(Operator::BNot)),

            (Regex::new(r"^in\b").unwrap(), Token::Operator(Operator::In)),
            (Regex::new(r"^(¬|~|n)in\b").unwrap(), Token::Operator(Operator::NIn)),

            (Regex::new(r"^(<|is_less_than\b)").unwrap(), Token::Operator(Operator::Les)),
            (Regex::new(r"^(>|is_greater_than\b)").unwrap(), Token::Operator(Operator::Gre)),
            (Regex::new(r"^(=|eq(uals)?\b)").unwrap(), Token::Operator(Operator::Equ)),
            (Regex::new(r"^(<|l)(=|eq(uals)?\b)").unwrap(), Token::Operator(Operator::LEq)),
            (Regex::new(r"^(>|g)(=|eq(uals)?\b)").unwrap(), Token::Operator(Operator::GEq)),
            (Regex::new(r"^(¬|~|n(ot_)?)(=|eq(uals)?\b)").unwrap(), Token::Operator(Operator::NEq)),


            (Regex::new(r"^\(~(\\.|[^~\\])*~\)").unwrap(), Token::Brace(Brace::Comment)),
            (Regex::new(r"^\(").unwrap(), Token::Brace(Brace::Round(false))),
            (Regex::new(r"^\)").unwrap(), Token::Brace(Brace::Round(true))),
            (Regex::new(r"^\[").unwrap(), Token::Brace(Brace::Square(false))),
            (Regex::new(r"^\]").unwrap(), Token::Brace(Brace::Square(true))),
            (Regex::new(r"^\{").unwrap(), Token::Brace(Brace::Curly(false))),
            (Regex::new(r"^\}").unwrap(), Token::Brace(Brace::Curly(true))),

            (Regex::new(r"^[A-Za-z][A-Za-z1-9\-_]*").unwrap(), Token::Identifier(String::new())),


            (Regex::new(r"^\s+").unwrap(), Token::WhiteSpace),
            (Regex::new(r".").unwrap(), Token::Unknown)
        ]
}
