use crate::error::Result;
use core::iter::Peekable;

// Lexing
//
// TODO: the `lex` function seems a bit clunky and probably be improved.

// > Usually one parses by first lexing the input and then constructing the parse tree.
// > The lex function gets a String and turns it into a vector of tokens. So first I
// > define another type for tokens.

// > I could have used more enum values to distinguish between + and * and the different
// > types of parentheses, but instead I just store the character. It probably would have
// > been a good idea to add another integer to each LexItem that stores the location in
// > the input at which the token starts. That would make error reporting more useful.
// TODO: add int to LexItem for debug purposes
//
// > Instead I will just use the position in the token stream for my errors. Since I’m
// > the only user of this program, I will only be angry at myself, so it’s okay.
#[derive(Debug, Clone, PartialEq)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}

// > The language I want to parse is very simple to lex. Except numbers, all tokens are
// > just a single character long. So instead of complicated things with regular
// > expressions. Instead I iterate over the characters of my input String and use a match
// > do create a LexItem.
fn lex(input: &str) -> Result<Vec<LexItem>> {
    let mut tokens = Vec::new();

    // need to consume multi-digit numbers
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        // for c in input.chars() {
        println!("read char: {}", c);
        match c {
            c @ '0'..='9' => {
                let token = consume_number(&mut it)?;
                tokens.push(token);
            }
            c @ '+' | c @ '*' => {
                tokens.push(LexItem::Op(c));
                it.next();
            }
            c @ '(' | c @ ')' | c @ '[' | c @ ']' | c @ '{' | c @ '}' => {
                tokens.push(LexItem::Paren(c));

                it.next();
            }
            ' ' => {
                //ignore white spaced
                it.next();
            }
            c => unreachable!("unexpected syntax: {c}"),
        };
    }

    Ok(tokens)
}

// the impl differs from the blog post.
fn consume_number<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Result<LexItem> {
    let mut number = String::new();
    while let Some(digit @ '0'..='9') = it.peek() {
        number.push(*digit);
        it.next();
    }

    if number.is_empty() {
        return Err(());
    }

    let token = LexItem::Num(
        number
            .parse::<u64>()
            .expect("failes to parse number: {number}"),
    );
    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_numbers() {
        assert_eq!(lex(" 123").unwrap(), vec![LexItem::Num(123)]);
        assert_eq!(lex(" 0").unwrap(), vec![LexItem::Num(0)]);
        assert_eq!(
            lex("1 23").unwrap(),
            vec![LexItem::Num(1), LexItem::Num(23)]
        );
        assert_eq!(lex("").unwrap(), vec![]);
    }

    #[test]
    fn test_lex() {
        assert_eq!(
            lex(" * + ").unwrap(),
            vec![LexItem::Op('*'), LexItem::Op('+')]
        );
        assert_eq!(
            lex("{ }( )[ ]").unwrap(),
            vec![
                LexItem::Paren('{'),
                LexItem::Paren('}'),
                LexItem::Paren('('),
                LexItem::Paren(')'),
                LexItem::Paren('['),
                LexItem::Paren(']')
            ]
        );
    }
}
