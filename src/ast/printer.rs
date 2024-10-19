use super::ASTVisitor;

use termion::color::Fg;
use termion::color::{self, White};

pub struct ASTTreePrinter {
    indentation: usize,
}

impl ASTTreePrinter {
    const INDENATION: usize = 2;

    const TEXT_COLOR: color::White = color::White;
    const STATEMENT_COLOR: color::Yellow = color::Yellow;
    const LET_STATEMENT_COLOR: color::Green = color::Green;
    const EXPR_COLOR: color::Green = color::Green;
    const BIN_EXPR_COLOR: color::LightBlue = color::LightBlue;
    const OPERATOR_COLOR: color::LightYellow = color::LightYellow;

    const STATEMENT_ICON: &str = nerd_font_symbols::md::MD_SIGMA;
    const LET_STATEMENT_ICON: &str = nerd_font_symbols::md::MD_EQUAL;
    const FUNC_STATEMENT_ICON: &str = nerd_font_symbols::md::MD_FUNCTION_VARIANT;
    const FUNC_CALL_STATEMENT_ICON: &str = nerd_font_symbols::md::MD_FUNCTION;
    const EXPR_ICON: &str = nerd_font_symbols::md::MD_FUNCTION_VARIANT;
    const BIN_EXPR_ICON: &str = nerd_font_symbols::cod::COD_SYMBOL_OPERATOR;
    const VARIABLE_ICON: &str = nerd_font_symbols::md::MD_VARIABLE;

    pub fn new() -> Self {
        Self { indentation: 0 }
    }

    fn increase_indentation(&mut self) {
        self.indentation += Self::INDENATION;
    }
    fn decrease_indentation(&mut self) {
        self.indentation -= Self::INDENATION;
    }

    fn print(&self, text: &str, text_color: &dyn color::Color) {
        // println!("{}├─ {}", "│ ".repeat(self.indentation), text);
        println!(
            "{}└─ {}{}{}",
            " ".repeat(self.indentation),
            color::Fg(text_color),
            text,
            color::Fg(color::Reset)
        );
    }
}

impl ASTVisitor for ASTTreePrinter {
    fn visit_statement(&mut self, statement: &super::ASTStatement) {
        self.print(
            &format!("{}  Statement:", Self::STATEMENT_ICON),
            &Self::STATEMENT_COLOR,
        );
        self.increase_indentation();
        ASTVisitor::do_visit_statement(self, statement);
        self.decrease_indentation();
    }

    fn visit_return_statement(&mut self, statement: &super::ASTReturnStatement) {
        self.print(
            &format!("{}  Return:", Self::LET_STATEMENT_ICON),
            &Self::LET_STATEMENT_COLOR,
        );
        self.increase_indentation();
        ASTVisitor::do_visit_expression(self, &statement.expr);
        self.decrease_indentation();
    }
    fn visit_let_statement(&mut self, statement: &super::ASTLetStatement) {
        self.print(
            &format!(
                "{}  Declaration: {}{}",
                Self::LET_STATEMENT_ICON,
                color::Fg(Self::TEXT_COLOR),
                &statement.identifier.span.literal
            ),
            &Self::LET_STATEMENT_COLOR,
        );
        self.increase_indentation();
        ASTVisitor::do_visit_expression(self, &statement.initializer);
        self.decrease_indentation();
    }
    fn visit_funtion_statement(&mut self, function: &super::ASTFunctionStatement) {
        self.print(
            &format!(
                "{}  Function: {}{}",
                Self::FUNC_STATEMENT_ICON,
                color::Fg(Self::TEXT_COLOR),
                &function.identifier.span.literal
            ),
            &Self::TEXT_COLOR,
        );

        self.increase_indentation();
        self.print(&format!("Arguments:"), &Self::TEXT_COLOR);
        self.increase_indentation();
        for arg in function.arguments.iter() {
            self.print(
                &format!(
                    "{}  Argument: {}{}",
                    Self::FUNC_STATEMENT_ICON,
                    color::Fg(Self::TEXT_COLOR),
                    &arg.identifier.span.literal
                ),
                &Self::TEXT_COLOR,
            );
        }
        self.decrease_indentation();

        self.print(&format!("Body:"), &Self::TEXT_COLOR);
        self.increase_indentation();

        if let super::ASTStatementKind::CompoundStatement(statement) = &function.body.kind {
            self.visit_compound_statement(statement);
        }
        self.decrease_indentation();
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        self.print(
            &format!(
                "{}  FunctionCall: {}{}",
                Self::FUNC_CALL_STATEMENT_ICON,
                color::Fg(Self::TEXT_COLOR),
                &expr.identifier.span.literal
            ),
            &Self::TEXT_COLOR,
        );
        self.increase_indentation();
        for expr in expr.arguments.iter() {
            ASTVisitor::do_visit_expression(self, &expr);
        }
        self.decrease_indentation();
    }

    fn visit_expression(&mut self, expr: &super::ASTExpression) {
        // self.print(
        //     &format!("{}  Expression:", Self::EXPR_ICON),
        //     &Self::EXPR_COLOR,
        // );
        // self.increase_indentation();
        ASTVisitor::do_visit_expression(self, &expr);
        // self.decrease_indentation();
    }

    fn visit_unary_expression(&mut self, expr: &super::ASTUnaryExpression) {
        self.print(
            &format!(
                "{}  Unary: {}{}",
                Self::BIN_EXPR_ICON,
                color::Fg(Self::OPERATOR_COLOR),
                expr.operator.token.span.literal
            ),
            &Self::BIN_EXPR_COLOR,
        );
        self.increase_indentation();
        self.visit_expression(&expr.expr);
        self.decrease_indentation();
    }

    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.print(
            &format!(
                "{}  Binary: {}{}",
                Self::BIN_EXPR_ICON,
                color::Fg(Self::OPERATOR_COLOR),
                expr.operator.token.span.literal
            ),
            &Self::BIN_EXPR_COLOR,
        );
        self.increase_indentation();
        // self.print_binary_operator(&expr.operator);
        // self.print(&format!("{:?}", expr.operator.kind), &Self::TEXT_COLOR);
        self.visit_expression(&expr.left);
        self.visit_expression(&expr.right);
        self.decrease_indentation();
    }

    fn visit_binary_operator(&mut self, op: &super::ASTBinaryOperator) {
        let var_name = format!(
            "Operator: {}",
            match op.kind {
                super::ASTBinaryOperatorKind::Plus => "+",
                super::ASTBinaryOperatorKind::Minus => "-",
                super::ASTBinaryOperatorKind::Multiply => "*",
                super::ASTBinaryOperatorKind::Divide => "/",
                super::ASTBinaryOperatorKind::EqualTo => "==",
                super::ASTBinaryOperatorKind::NotEqualTo => "!=",
                super::ASTBinaryOperatorKind::LogicAND => "&&",
                super::ASTBinaryOperatorKind::LogicOR => "||",
                super::ASTBinaryOperatorKind::GreaterThan => ">",
                super::ASTBinaryOperatorKind::GreaterThanOrEqual => ">=",
                super::ASTBinaryOperatorKind::LessThan => "<",
                super::ASTBinaryOperatorKind::LessThanOrEqual => "<=",
                super::ASTBinaryOperatorKind::BitwiseOR => "|",
                super::ASTBinaryOperatorKind::BitwiseAND => "&",
                super::ASTBinaryOperatorKind::BitwiseXOR => "^",
            }
        );
        self.print(&var_name, &color::Yellow);
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        self.print(
            &format!(
                "{}  Parenthesized:",
                nerd_font_symbols::md::MD_CODE_PARENTHESES
            ),
            &color::Magenta,
        );
        self.increase_indentation();
        self.visit_expression(&expr.expr);
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        self.print(
            &format!("{}  Variable: {}", Self::VARIABLE_ICON, expr.identifier()),
            &Self::TEXT_COLOR,
        );
    }
    fn visit_error(&mut self, span: &super::TextSpan) {
        self.print(&format!("Error: {:?}", span), &color::Red);
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.print(&format!("Integer: {}", integer), &Self::TEXT_COLOR);
    }

    fn visit_float(&mut self, float: &f64) {
        self.print(&format!("Float: {}", float), &Self::TEXT_COLOR);
    }

    fn visit_conditional_statement(&mut self, statement: &super::ASTConditionalStatement) {
        self.print(&format!("If:"), &color::Blue);
        self.increase_indentation();
        self.visit_expression(&statement.codition);
        self.increase_indentation();
        self.visit_statement(&statement.then_branch);
        if let Some(else_branch) = &statement.else_branch {
            self.visit_statement(&else_branch.else_branch);
        }
    }
}

pub struct ASTHiglightPrinter {
    indent: usize,
    result: String,
}

impl ASTHiglightPrinter {
    const INDENATION: usize = 2;
    const INTEGER_COLOR: color::Cyan = color::Cyan;
    const FLOAT_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::White = color::White;
    const LET_COLOR: color::Red = color::Red;
    const FUNC_COLOR: color::Red = color::Red;
    const FUNC_CALL_COLOR: color::Yellow = color::Yellow;
    const FUNC_NAME_COLOR: color::Yellow = color::Yellow;
    const VARIABLE_COLOR: color::LightGreen = color::LightGreen;

    pub fn new() -> Self {
        Self {
            indent: 0,
            result: "".to_string(),
        }
    }

    pub fn print_result(&self) {
        println!("Highlighted Source:\n{}{}", self.result, Fg(White));
    }

    fn add_whitespace(&mut self) {
        self.print(" ");
    }

    fn add_newline(&mut self) {
        self.print("\n");
    }

    fn increase_indentation(&mut self) {
        self.indent += Self::INDENATION;
    }
    fn decrease_indentation(&mut self) {
        self.indent -= Self::INDENATION;
    }

    fn print_indent(&mut self) {
        self.result.push_str(&format!(
            "{}{}",
            " ".repeat(self.indent),
            color::Fg(color::Reset)
        ));
    }
    fn print(&mut self, text: &str) {
        self.result
            .push_str(&format!("{}{}", text, color::Fg(color::Reset)));
    }

    fn visit_idenifier(&mut self, identifier: &String) {
        self.print(&format!("{}{}", Fg(Self::TEXT_COLOR), identifier));
    }
}

impl ASTVisitor for ASTHiglightPrinter {
    fn visit_statement(&mut self, statement: &super::ASTStatement) {
        self.print_indent();
        self.do_visit_statement(statement);
    }
    fn visit_return_statement(&mut self, statement: &super::ASTReturnStatement) {
        self.print(&format!("{}return", Fg(Self::LET_COLOR)));
        self.add_whitespace();
        self.visit_expression(&statement.expr);
    }

    fn visit_let_statement(&mut self, statement: &super::ASTLetStatement) {
        self.print(&format!("{}let", Fg(Self::LET_COLOR)));
        self.add_whitespace();
        self.visit_idenifier(&statement.identifier.span.literal);
        self.add_whitespace();
        self.print(&format!("{}=", Fg(Self::TEXT_COLOR)));
        self.add_whitespace();
        self.visit_expression(&statement.initializer);
        self.add_newline();
    }

    fn visit_compound_statement(&mut self, statement: &super::ASTCompoundStatement) {
        self.print(&format!("{}{}", Fg(Self::TEXT_COLOR), '{'));
        self.add_newline();
        self.increase_indentation();
        for statement in statement.statements.iter() {
            self.visit_statement(statement);
        }
        self.add_newline();
        self.decrease_indentation();
        self.print_indent();
        self.print(&format!("{}{}", Fg(Self::TEXT_COLOR), '}'));
    }

    fn visit_conditional_statement(&mut self, statement: &super::ASTConditionalStatement) {
        self.print(&format!(
            "{}if {}",
            Fg(Self::FUNC_COLOR),
            Fg(Self::TEXT_COLOR),
        ));
        self.visit_expression(&statement.codition);
        // self.increase_indentation();
        // self.increase_indentation();
        self.visit_statement(&statement.then_branch);
        if let Some(else_branch) = &statement.else_branch {
            self.print(&format!(
                "{}else{} ",
                Fg(Self::FUNC_COLOR),
                Fg(Self::TEXT_COLOR),
            ));
            self.visit_statement(&else_branch.else_branch);
        }
        self.add_newline();
    }

    fn visit_funtion_statement(&mut self, function: &super::ASTFunctionStatement) {
        self.print(&format!(
            "{}func {}{}{}(",
            Fg(Self::FUNC_COLOR),
            Fg(Self::FUNC_NAME_COLOR),
            function.identifier.span.literal,
            Fg(Self::TEXT_COLOR),
        ));
        for (i, arg) in function.arguments.iter().enumerate() {
            if i != 0 {
                self.print(&format!("{},", Fg(Self::TEXT_COLOR)));
                self.add_whitespace();
            }
            self.print(&format!(
                "{}{}",
                Fg(Self::TEXT_COLOR),
                arg.identifier.span.literal
            ));
        }

        self.print(&format!("{}) ", Fg(Self::TEXT_COLOR)));
        if let super::ASTStatementKind::CompoundStatement(statement) = &function.body.kind {
            self.visit_compound_statement(statement);
        }
        self.add_newline();
    }

    fn visit_function_call_expression(&mut self, expr: &super::ASTFunctionCallExpression) {
        self.print(&format!(
            "{}{}{}(",
            Fg(Self::FUNC_CALL_COLOR),
            expr.identifier(),
            Fg(Self::TEXT_COLOR)
        ));

        for (i, arg) in expr.arguments.iter().enumerate() {
            if i != 0 {
                self.print(&format!("{},", Fg(Self::TEXT_COLOR)));
                self.add_whitespace();
            }
            self.visit_expression(arg);
        }
        self.print(&format!("{})", Fg(Self::TEXT_COLOR)));
        self.add_whitespace();
    }

    fn visit_variable_expression(&mut self, expr: &super::ASTVariableExpression) {
        self.print(&format!(
            "{}{}",
            Fg(Self::VARIABLE_COLOR),
            expr.identifier()
        ));
    }

    fn visit_unary_expression(&mut self, expr: &super::ASTUnaryExpression) {
        self.print(&format!(
            "{}{}",
            Fg(Self::TEXT_COLOR),
            expr.operator.token.span.literal
        ));
        self.visit_expression(&expr.expr);
    }

    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        self.add_whitespace();
        self.print(&format!(
            "{}{}",
            Fg(Self::TEXT_COLOR),
            expr.operator.token.span.literal
        ));
        self.add_whitespace();
        self.visit_expression(&expr.right);
    }

    fn visit_parenthesised_expression(&mut self, expr: &super::ASTParenthesizedExpression) {
        self.print(&format!("{}(", Fg(Self::TEXT_COLOR)));
        self.visit_expression(&expr.expr);
        self.print(&format!("{})", Fg(Self::TEXT_COLOR)));
    }

    fn visit_integer(&mut self, integer: &i64) {
        self.print(&format!("{}{}", Fg(Self::INTEGER_COLOR), integer));
    }
    fn visit_float(&mut self, float: &f64) {
        self.print(&format!("{}{}", Fg(Self::FLOAT_COLOR), float));
    }

    fn visit_binary_operator(&mut self, op: &super::ASTBinaryOperator) {
        self.print(&format!(
            "{}{}",
            Fg(Self::TEXT_COLOR),
            match op.kind {
                super::ASTBinaryOperatorKind::Plus => "+",
                super::ASTBinaryOperatorKind::Minus => "-",
                super::ASTBinaryOperatorKind::Multiply => "*",
                super::ASTBinaryOperatorKind::Divide => "/",
                super::ASTBinaryOperatorKind::EqualTo => "==",
                super::ASTBinaryOperatorKind::NotEqualTo => "!=",
                super::ASTBinaryOperatorKind::LogicAND => "&&",
                super::ASTBinaryOperatorKind::LogicOR => "||",
                super::ASTBinaryOperatorKind::GreaterThan => ">",
                super::ASTBinaryOperatorKind::GreaterThanOrEqual => ">=",
                super::ASTBinaryOperatorKind::LessThan => "<",
                super::ASTBinaryOperatorKind::LessThanOrEqual => "<=",
                super::ASTBinaryOperatorKind::BitwiseOR => "|",
                super::ASTBinaryOperatorKind::BitwiseAND => "&",
                super::ASTBinaryOperatorKind::BitwiseXOR => "^",
            }
        ));
    }
}
