use std::borrow::BorrowMut;

use crate::ast::lexer::{Lexer, Token, TokenKind};
use crate::ast::{ASTExpression, ASTStatement};
use crate::diagnostics::DiagnosticsColletionCell;

use super::{ASTBinaryExpression, ASTBinaryOperator, ASTBinaryOperatorKind};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    diagnostics_colletion: DiagnosticsColletionCell,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, diagnostics_colletion: DiagnosticsColletionCell) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter(|token| token.kind != TokenKind::Whitespace)
                .map(|token| token.clone())
                .collect(),
            cursor: 0,
            diagnostics_colletion,
        }
    }

    pub fn from_input(input: String, diagnostics_colletion: DiagnosticsColletionCell) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            if token.kind != TokenKind::Whitespace {
                tokens.push(token);
            }
        }
        Self {
            tokens,
            cursor: 0,
            diagnostics_colletion,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        if self.current_token().kind == TokenKind::Eof {
            return None;
        }
        Some(self.parse_statement())
    }

    fn parse_statement(&mut self) -> ASTStatement {
        let expr = self.parse_expression();
        ASTStatement::expression(expr)
    }

    fn current_token(&self) -> &Token {
        self.peek(0)
    }

    fn peek(&self, offset: isize) -> &Token {
        let index = std::cmp::min(
            (self.cursor as isize + offset) as usize,
            self.tokens.len() - 1,
        );
        self.tokens.get(index).unwrap()
    }

    fn consume(&mut self) -> &Token {
        self.cursor += 1;
        println!("{:?}", self.peek(-1));
        self.peek(-1)
    }

    fn consume_expected(&mut self, expected: TokenKind) -> &Token {
        let token = self.consume();
        if token.kind != expected {
            println!("VERFLIXT {:?}", token);
            // self.diagnostics_colletion
            //     .borrow_mut()
            //     .report_unexpected_token(&expected, token);
        }
        token
    }

    fn parse_expression(&mut self) -> ASTExpression {
        self.parse_binary_expression(0)
    }

    fn parse_primary_expression(&mut self) -> ASTExpression {
        let token = self.consume();
        match token.kind {
            TokenKind::Integer(i) => ASTExpression::integer(i),
            TokenKind::Floating(i) => ASTExpression::float(i),
            TokenKind::LeftParen => {
                let expr = self.parse_binary_expression(0);
                let found_token = self.consume_expected(TokenKind::RightParen);
                // return ASTExpression::error(found_token.span.clone());
                expr
            }
            _ => {
                // self.diagnostics_colletion
                //     .get_mut()
                //     .report_expected_expression(token);
                ASTExpression::error(token.span.clone())
            }
        }
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> ASTExpression {
        let mut left = self.parse_primary_expression();

        while let Some(operator) = self.parse_binary_operator() {
            self.consume();

            let operator_precedence = operator.precedence();
            if operator_precedence >= precedence {
                let right = self.parse_binary_expression(operator_precedence);
                left = ASTExpression::binary(operator, left, right);
            } else {
                break;
            }
        }
        left
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.current_token();
        let kind = match token.kind {
            TokenKind::Plus => Some(ASTBinaryOperatorKind::Plus),
            TokenKind::Minus => Some(ASTBinaryOperatorKind::Minus),
            TokenKind::Astrisk => Some(ASTBinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(ASTBinaryOperatorKind::Divide),
            _ => None,
        };
        kind.map(|kind| {
            return ASTBinaryOperator {
                kind,
                token: token.clone(),
            };
        })
    }
}
