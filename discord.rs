#[derive(Default, Debug)]
pub struct Parser {
    pub ast: ast::Ast,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            ..Default::default()
        }
    }

    pub fn check(&mut self, lexer: &Lexer) -> Result<()> {
        let mut tokens = lexer.tokens.iter().peekable();

        while tokens.peek().is_some() {
            let statement = self.statement(&mut tokens)?;
            self.ast.program.push(statement);
        }
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    fn statement<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token> + std::fmt::Debug,
    {
        match tokens.peek() {
            Some(Token::NEWLINE) => {
                tokens.next();
                Ok(self.statement(tokens)?)
            } // ...
        }
    }
}

#[derive(Default, Debug)]
pub struct Parser {
    pub ast: ast::Ast,
    iter: Box<None>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            ..Default::default()
        }
    }

    pub fn check(&mut self, lexer: &Lexer) -> Result<()> {
        let mut tokens = lexer.tokens.iter().peekable();

        while tokens.peek().is_some() {
            let statement = self.statement(&mut tokens)?;
            self.ast.program.push(statement);
        }
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    fn statement<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token> + std::fmt::Debug,
    {
        match tokens.peek() {
            Some(Token::NEWLINE) => {
                tokens.next();
                Ok(self.statement(tokens)?)
            } // ...
        }
    }
}
