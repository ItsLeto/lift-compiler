use crate::ast::lexer::{Lexer, Token, TokenKind};
use crate::ast::{ASTExpression, ASTStatement};
use crate::diagnostics::DiagnosticsColletion;
use crate::diagnostics::DiagnosticsColletionCell;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use super::{
    ASTBinaryOperator, ASTBinaryOperatorKind, ASTElseStatement, ASTUnaryOperator,
    ASTUnaryOperatorKind, FunctionArgumentDeclaration,
};

struct Cursor {
    cursor: Cell<usize>,
}

impl Cursor {
    fn new() -> Self {
        Self {
            cursor: Cell::new(0),
        }
    }

    fn move_forward(&self) {
        let value = self.cursor.get();
        self.cursor.set(value + 1);
    }

    fn get_value(&self) -> usize {
        self.cursor.get()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: Cursor,
    diagnostics_colletion: DiagnosticsColletionCell,
}

impl Parser {
    pub fn new(
        tokens: Vec<Token>,
        diagnostics_colletion: Rc<RefCell<DiagnosticsColletion>>,
    ) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter(|token| {
                    token.kind != TokenKind::Whitespace && token.kind != TokenKind::SemiColon
                })
                .map(|token| token.clone())
                .collect(),
            cursor: Cursor::new(),
            diagnostics_colletion,
        }
    }

    pub fn from_input(input: String, diagnostics_colletion: DiagnosticsColletionCell) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            if token.kind != TokenKind::Whitespace && token.kind != TokenKind::SemiColon {
                tokens.push(token);
            }
        }
        Self {
            tokens,
            cursor: Cursor::new(),
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
        match self.current_token().kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::Func => self.parse_function_statement(),
            TokenKind::If => self.parse_conditional_statement(),
            TokenKind::LeftBrace => self.parse_compound_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn current_token(&self) -> &Token {
        self.peek(0)
    }

    fn peek(&self, offset: isize) -> &Token {
        let index = std::cmp::min(
            (self.cursor.get_value() as isize + offset) as usize,
            self.tokens.len() - 1,
        );
        self.tokens.get(index).unwrap()
    }

    fn consume(&self) -> &Token {
        self.cursor.move_forward();
        self.peek(-1)
    }

    fn consume_expected(&self, expected: TokenKind) -> &Token {
        let token = self.consume();
        if token.kind != expected {
            self.diagnostics_colletion
                .borrow_mut()
                .report_unexpected_token(&expected, token);
        }
        token
    }

    fn parse_return_statement(&mut self) -> ASTStatement {
        self.consume_expected(TokenKind::Return);
        let expr = self.parse_expression();
        ASTStatement::return_statement(expr)
    }

    fn parse_let_statement(&mut self) -> ASTStatement {
        self.consume_expected(TokenKind::Let);
        let identifier = self.consume_expected(TokenKind::Identifier).clone();
        self.consume_expected(TokenKind::Equal);
        let expr = self.parse_expression();
        ASTStatement::let_statement(identifier, expr)
    }

    fn parse_compound_statement(&mut self) -> ASTStatement {
        self.consume_expected(TokenKind::LeftBrace);
        let mut statements: Vec<ASTStatement> = Vec::new();
        while self.current_token().kind != TokenKind::RightBrace {
            println!("{:?}", self.current_token());
            statements.push(self.parse_statement());
        }
        self.consume_expected(TokenKind::RightBrace);
        ASTStatement::compound(statements)
    }

    fn parse_function_statement(&mut self) -> ASTStatement {
        self.consume_expected(TokenKind::Func);
        let identifier = self.consume_expected(TokenKind::Identifier).clone();
        self.consume_expected(TokenKind::LeftParen);

        if self.current_token().kind == TokenKind::Comma {
            self.diagnostics_colletion
                .borrow_mut()
                .report_unexpected_token(&TokenKind::Identifier, self.peek(1));
            self.consume();
        }

        let mut arguments: Vec<FunctionArgumentDeclaration> = Vec::new();
        while self.current_token().kind != TokenKind::RightParen {
            if self.current_token().kind == TokenKind::Comma {
                self.diagnostics_colletion
                    .borrow_mut()
                    .report_unexpected_token(&TokenKind::Identifier, self.current_token());
                self.consume();
            }

            if self.current_token().kind == TokenKind::Identifier {
                arguments.push(FunctionArgumentDeclaration {
                    identifier: self.consume().clone(),
                });
            } else {
                self.diagnostics_colletion
                    .borrow_mut()
                    .report_unexpected_token(&TokenKind::Identifier, self.current_token());
            }

            if self.current_token().kind == TokenKind::Comma
                && self.peek(1).kind == TokenKind::RightParen
            {
                self.consume_expected(TokenKind::RightParen);
                break;
            } else if self.current_token().kind == TokenKind::Comma {
                self.consume(); // Consume comma if present
            }
        }

        self.consume_expected(TokenKind::RightParen);
        let body = self.parse_compound_statement();

        ASTStatement::function(identifier, arguments, body)
    }

    fn consume_optional_else_statement(&mut self) -> Option<ASTElseStatement> {
        if self.current_token().kind != TokenKind::Else {
            return None;
        }
        let else_keyword = self.consume_expected(TokenKind::Else).clone();
        let else_branch = self.parse_statement();
        Some(ASTElseStatement {
            else_keyword: else_keyword,
            else_branch: Box::new(else_branch),
        })
    }

    fn parse_conditional_statement(&mut self) -> ASTStatement {
        let keyword = self.consume_expected(TokenKind::If).clone();
        self.consume_expected(TokenKind::LeftParen);
        let condition = self.parse_expression();
        self.consume_expected(TokenKind::RightParen);
        let then_branch = self.parse_statement();
        let else_branch = self.consume_optional_else_statement();

        ASTStatement::conditional(keyword, condition, then_branch, else_branch)
    }

    fn parse_expression_statement(&mut self) -> ASTStatement {
        let expr = self.parse_expression();
        ASTStatement::expression(expr)
    }

    fn parse_assignment_expression(&mut self) -> ASTExpression {
        if self.current_token().kind == TokenKind::Identifier
            && self.peek(1).kind == TokenKind::Equal
        {
            let var = self.consume().clone();
            self.consume_expected(TokenKind::Equal);
            let assignment = self.parse_binary_expression(0);
            return ASTExpression::assignment(var, assignment);
        }
        self.parse_binary_expression(0)
    }

    fn parse_expression(&mut self) -> ASTExpression {
        self.parse_assignment_expression()
    }

    fn parse_arguments_list(&mut self) -> Vec<ASTExpression> {
        if self.current_token().kind == TokenKind::Comma {
            self.diagnostics_colletion
                .borrow_mut()
                .report_unexpected_token(&TokenKind::Identifier, self.peek(1));
            self.consume();
        }

        let mut arguments: Vec<ASTExpression> = Vec::new();
        while self.current_token().kind != TokenKind::RightParen {
            if self.current_token().kind == TokenKind::Comma {
                self.diagnostics_colletion
                    .borrow_mut()
                    .report_unexpected_token(&TokenKind::Identifier, self.current_token());
                self.consume();
            }
            arguments.push(self.parse_expression());
            if self.current_token().kind == TokenKind::Comma
                && self.peek(1).kind == TokenKind::RightParen
            {
                self.consume_expected(TokenKind::RightParen);
                break;
            } else if self.current_token().kind == TokenKind::Comma {
                self.consume(); // Consume comma if present
            }
        }
        arguments
    }

    fn parse_function_call_expression(&mut self) -> ASTExpression {
        let identifier = self.peek(-1).clone();
        self.consume();
        let arguments = self.parse_arguments_list();
        self.consume_expected(TokenKind::RightParen);
        ASTExpression::function_call(identifier.clone(), arguments)
    }

    fn parse_primary_expression(&mut self) -> ASTExpression {
        let token = self.consume().clone();

        return match token.kind {
            TokenKind::Integer(i) => ASTExpression::integer(i),
            TokenKind::Floating(i) => ASTExpression::float(i),
            TokenKind::Identifier => {
                if self.current_token().kind == TokenKind::LeftParen {
                    self.parse_function_call_expression()
                } else {
                    ASTExpression::identifier(token.clone())
                }
            }

            TokenKind::LeftParen => {
                let expr = self.parse_binary_expression(0);
                let _found_token = self.consume_expected(TokenKind::RightParen);
                ASTExpression::parenthesized(expr)
            }
            TokenKind::Tilde | TokenKind::Minus | TokenKind::ExclemationMark => {
                self.parse_unary_expression()
            }
            _ => {
                self.diagnostics_colletion
                    .borrow_mut()
                    .report_expected_expression(&token);
                ASTExpression::error(token.span)
            }
        };
    }

    fn parse_unary_expression(&mut self) -> ASTExpression {
        let operator = self.parse_unary_operator().unwrap();
        let expr = self.parse_primary_expression();
        ASTExpression::unary(operator, expr)
    }
    fn parse_binary_expression(&mut self, precedence: u8) -> ASTExpression {
        let mut left = self.parse_primary_expression();

        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence > precedence {
                self.consume();
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

            TokenKind::Pipe => Some(ASTBinaryOperatorKind::BitwiseOR),
            TokenKind::Ampersand => Some(ASTBinaryOperatorKind::BitwiseAND),
            TokenKind::Caret => Some(ASTBinaryOperatorKind::BitwiseXOR),
            TokenKind::EqualEqual => Some(ASTBinaryOperatorKind::EqualTo),
            TokenKind::ExclemationMarkEqual => Some(ASTBinaryOperatorKind::NotEqualTo),
            TokenKind::AmpersandAmpersand => Some(ASTBinaryOperatorKind::LogicAND),
            TokenKind::PipePipe => Some(ASTBinaryOperatorKind::LogicOR),
            TokenKind::RightAngleBracket => Some(ASTBinaryOperatorKind::GreaterThan),
            TokenKind::RightAngleBracketEqual => Some(ASTBinaryOperatorKind::GreaterThanOrEqual),
            TokenKind::LeftAngleBracket => Some(ASTBinaryOperatorKind::LessThan),
            TokenKind::LeftAngleBracketEqual => Some(ASTBinaryOperatorKind::LessThanOrEqual),
            _ => None,
        };
        kind.map(|kind| {
            return ASTBinaryOperator {
                kind,
                token: token.clone(),
            };
        })
    }

    fn parse_unary_operator(&mut self) -> Option<ASTUnaryOperator> {
        let token = self.current_token();
        let kind = match token.kind {
            TokenKind::Tilde => Some(ASTUnaryOperatorKind::BitwiseNOT),
            TokenKind::ExclemationMark => Some(ASTUnaryOperatorKind::LogicNot),
            TokenKind::Minus => Some(ASTUnaryOperatorKind::Minus),
            _ => None,
        };
        kind.map(|kind| {
            return ASTUnaryOperator {
                kind,
                token: token.clone(),
            };
        })
    }
}
