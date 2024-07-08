use crate::error::Result;
use crate::error::SimpleError;
use crate::grammar::GrammarItem;
use crate::grammar::ParseNode;
use crate::lexer;
use crate::lexer::LexItem;

// Not sure how exactly to read this grammar. Need to understand the notation.
// http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp
//
// wayback link: https://web.archive.org/web/20240603005149/https://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp
//
// > The grammar I came up with is as follows:
// >    expr -> summand + expr | summand
// >    summand -> term * summand | term
// >    term -> NUMBER | ( expr )
//
// So this is CFG or context free grammar and is used to define programming languages.

pub fn parse(input: &str) -> Result<ParseNode> {
    let tokens = lexer::lex(input)?;
    let (node, idx) = parse_expr(&tokens, 0)?;
    if idx == tokens.len() {
        Ok(node)
    } else {
        let err = SimpleError::Parser(format!(
            "expected end of input, found {:?} at {idx}",
            tokens[idx]
        ));
        Err(err)
    }
}

// >    expr -> summand + expr | summand
fn parse_expr(tokens: &[LexItem], idx: usize) -> Result<(ParseNode, usize)> {
    let (node_summand, nxt_idx) = parse_summand(tokens, idx)?;

    if let Some(&LexItem::Op('+')) = tokens.get(nxt_idx) {
        // resurse on the expr
        let mut sum = ParseNode::new(GrammarItem::Sum, Some(node_summand));
        let (rhs, nxt_idx) = parse_expr(tokens, nxt_idx + 1)?;
        sum.children.push(rhs);
        Ok((sum, nxt_idx))
    } else {
        // > we have just the summand production, nothing more
        //
        // TODO: the above comment is from the blog and I am not sure what it means
        Ok((node_summand, nxt_idx))
    }
}

// >    summand -> term * summand | term
fn parse_summand(tokens: &[LexItem], idx: usize) -> Result<(ParseNode, usize)> {
    let (node_term, nxt_idx) = parse_term(tokens, idx)?;

    if let Some(&LexItem::Op('*')) = tokens.get(nxt_idx) {
        // resurse on the summand
        let mut product = ParseNode::new(GrammarItem::Product, Some(node_term));
        let (rhs, nxt_idx) = parse_summand(tokens, nxt_idx + 1)?;
        product.children.push(rhs);
        Ok((product, nxt_idx))
    } else {
        // > we have just the term production, nothing more.
        //
        // TODO: the above comment is from the blog and I am not sure what it means
        Ok((node_term, nxt_idx))
    }
}

// >    term -> NUMBER | ( expr )
fn parse_term(tokens: &[LexItem], idx: usize) -> Result<(ParseNode, usize)> {
    let token = tokens
        .get(idx)
        .expect("unexpected end of input.. expected paren or number");
    let out = match token {
        LexItem::Num(n) => Ok((ParseNode::new(GrammarItem::Number(*n), None), idx + 1)),
        LexItem::Paren(open) => {
            if *open != '(' && *open != '[' && *open != '{' {
                return Err(SimpleError::Parser(format!(
                    "expected paren at {idx} but found {open}"
                )));
            }

            let (node, nxt_idx) = parse_expr(tokens, idx + 1)?;
            let close = tokens.get(nxt_idx);
            if let Some(&LexItem::Paren(close)) = close {
                if close == matching(open) {
                    // correctly matched open and closing paren
                    let paren = ParseNode::new(GrammarItem::Paren, Some(node));
                    Ok((paren, nxt_idx + 1))
                } else {
                    Err(SimpleError::Parser(
                        "expected closing paren at {idx} but found {closing}".to_owned(),
                    ))
                }
            } else {
                Err(SimpleError::Parser(
                    "expected closing paren at {idx} but found {closing}".to_owned(),
                ))
            }
        }
        LexItem::Op(opt) => Err(SimpleError::Parser(format!(
            "Unexpected token: {opt}, expected paren or number"
        ))),
    };

    out
}

fn matching(c: &char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => panic!("should have been a parenthesis!"),
    }
}

#[allow(clippy::get_first)]
pub fn format_pretty(tree: &ParseNode) -> String {
    match tree.entry {
        GrammarItem::Paren => {
            format!(
                "({})",
                format_pretty(tree.children.get(0).expect("parens need one child"))
            )
        }
        GrammarItem::Sum => {
            let lhs = format_pretty(tree.children.get(0).expect("sums need two children"));
            let rhs = format_pretty(tree.children.get(1).expect("sums need two children"));
            format!("{} + {}", lhs, rhs)
        }
        GrammarItem::Product => {
            let lhs = format_pretty(tree.children.get(0).expect("products need two children"));
            let rhs = format_pretty(tree.children.get(1).expect("products need two children"));
            format!("{} * {}", lhs, rhs)
        }
        GrammarItem::Number(n) => format!("{}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // ParseNode {
        //     entry: Sum,
        //     children: [
        //         ParseNode {
        //             entry: Number(1),
        //             children: []
        //         },
        //         ParseNode {
        //             entry: Number(2),
        //             children: []
        //         }
        //     ]
        // }
        let tree = parse("1+2").unwrap();
        println!("{:?}", tree);

        // ParseNode {
        //     entry: Sum,
        //     children: [
        //         ParseNode {
        //             entry: Number(1234),
        //             children: []
        //          },
        //          ParseNode {
        //              entry: Product,
        //              children: [
        //                  ParseNode {
        //                      entry: Number(43),
        //                      children: []
        //                  },
        //                  ParseNode {
        //                      entry: Paren,
        //                      children: [
        //                          ParseNode {
        //                              entry: Sum,
        //                              children: [
        //                                  ParseNode {
        //                                      entry: Number(34),
        //                                      children: []
        //                                  },
        //                                  ParseNode {
        //                                      entry: Paren,
        //                                      children: [
        //                                          ParseNode {
        //                                              entry: Number(2),
        //                                              children: []
        //                                          }
        //                                      ]
        //                                  }
        //                              ]
        //                          }
        //                      ]
        //                  }
        //              ]
        //          }
        //     ]
        // }
        let tree = parse("1234 + 43* (34 +[2])").unwrap();
        println!("{:?}", tree);
    }
}
