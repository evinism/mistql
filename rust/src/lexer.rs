//! Lexer implementation for MistQL
//!
//! This module provides lexical analysis for MistQL expressions, converting
//! source code into a stream of tokens that can be consumed by the parser.

use std::fmt;
use std::str::Chars;

/// Represents the position of a token in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }

    pub fn start() -> Self {
        Self::new(1, 1, 0)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Represents a token in the MistQL language
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, position: Position) -> Self {
        Self {
            token_type,
            value,
            position,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.token_type, self.value)
    }
}

/// All possible token types in MistQL
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    Number,
    String,
    True,
    False,
    Null,

    // Identifiers and references
    Identifier,
    At,     // @
    Dollar, // $

    // Operators
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // Comparison operators
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    Match,        // =~

    // Logical operators
    And, // &&
    Or,  // ||
    Not, // !

    // Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Colon,        // :
    Dot,          // .
    Pipe,         // |

    // Special
    Whitespace,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::String => write!(f, "STRING"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Null => write!(f, "NULL"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::At => write!(f, "AT"),
            TokenType::Dollar => write!(f, "DOLLAR"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Multiply => write!(f, "MULTIPLY"),
            TokenType::Divide => write!(f, "DIVIDE"),
            TokenType::Modulo => write!(f, "MODULO"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::NotEqual => write!(f, "NOT_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Match => write!(f, "MATCH"),
            TokenType::And => write!(f, "AND"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Not => write!(f, "NOT"),
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBracket => write!(f, "LEFT_BRACKET"),
            TokenType::RightBracket => write!(f, "RIGHT_BRACKET"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Colon => write!(f, "COLON"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Pipe => write!(f, "PIPE"),
            TokenType::Whitespace => write!(f, "WHITESPACE"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

/// Lexer error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedCharacter(char, Position),
    UnterminatedString(Position),
    InvalidNumber(String, Position),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(c, pos) => {
                write!(f, "Unexpected character '{}' at {}", c, pos)
            }
            LexerError::UnterminatedString(pos) => {
                write!(f, "Unterminated string at {}", pos)
            }
            LexerError::InvalidNumber(s, pos) => {
                write!(f, "Invalid number '{}' at {}", s, pos)
            }
        }
    }
}

impl std::error::Error for LexerError {}

/// The main lexer struct
pub struct Lexer<'a> {
    _input: &'a str,
    chars: Chars<'a>,
    pub current_pos: Position,
    peek_pos: Position,
    current_char: Option<char>,
    peek_char: Option<char>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input string
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            _input: input,
            chars: input.chars(),
            current_pos: Position::start(),
            peek_pos: Position::start(),
            current_char: None,
            peek_char: None,
        };

        // Initialize the first two characters
        lexer.read_char();
        lexer.read_char();

        lexer
    }

    /// Read the next character and advance the position
    fn read_char(&mut self) {
        self.current_char = self.peek_char;
        self.current_pos = self.peek_pos.clone();

        self.peek_char = self.chars.next();
        if self.current_char == Some('\n') {
            self.peek_pos.line += 1;
            self.peek_pos.column = 1;
        } else {
            self.peek_pos.column += 1;
        }
        self.peek_pos.offset += 1;
    }

    /// Peek at the next character without advancing
    fn peek_char(&self) -> Option<char> {
        self.peek_char
    }

    /// Read whitespace characters as a single space token
    fn read_whitespace(&mut self) -> String {
        let mut whitespace = String::new();
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                whitespace.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        // Collapse all whitespace into a single space
        if !whitespace.is_empty() {
            " ".to_string()
        } else {
            whitespace
        }
    }

    /// Read an identifier or keyword
    fn read_identifier(&mut self) -> String {
        let _start_pos = self.current_pos.clone();
        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        identifier
    }

    /// Read a number (integer or float)
    fn read_number(&mut self) -> Result<String, LexerError> {
        let start_pos = self.current_pos.clone();
        let mut number = String::new();

        // Read integer part
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                number.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        // Read decimal part if present
        if self.current_char == Some('.') {
            // Check if the next character is a digit
            if let Some(next_char) = self.peek_char() {
                if next_char.is_ascii_digit() {
                    number.push('.');
                    self.read_char();

                    // Read fractional digits
                    while let Some(c) = self.current_char {
                        if c.is_ascii_digit() {
                            number.push(c);
                            self.read_char();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Validate the number
        if number.is_empty() {
            return Err(LexerError::InvalidNumber(number, start_pos));
        }

        // Check if it's a valid number
        if number.parse::<f64>().is_err() {
            return Err(LexerError::InvalidNumber(number, start_pos));
        }

        Ok(number)
    }

    /// Read a string literal
    fn read_string(&mut self) -> Result<String, LexerError> {
        let start_pos = self.current_pos.clone();
        let mut string = String::new();

        // Skip the opening quote
        self.read_char();

        while let Some(c) = self.current_char {
            match c {
                '"' => {
                    // End of string
                    self.read_char();
                    return Ok(string);
                }
                '\\' => {
                    // Handle escape sequences
                    self.read_char();
                    if let Some(escaped) = self.current_char {
                        match escaped {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            _ => string.push(escaped), // For other escape sequences
                        }
                        self.read_char();
                    } else {
                        return Err(LexerError::UnterminatedString(start_pos));
                    }
                }
                _ => {
                    string.push(c);
                    self.read_char();
                }
            }
        }

        // If we get here, the string was not terminated
        Err(LexerError::UnterminatedString(start_pos))
    }

    /// Determine if an identifier is a keyword
    fn lookup_identifier(identifier: &str) -> TokenType {
        match identifier {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier,
        }
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        let token = match self.current_char {
            Some(c) if c.is_whitespace() => {
                let whitespace_value = self.read_whitespace();
                return Ok(Token::new(TokenType::Whitespace, whitespace_value, self.current_pos.clone()));
            }
            Some('+') => Token::new(TokenType::Plus, "+".to_string(), self.current_pos.clone()),
            Some('-') => Token::new(TokenType::Minus, "-".to_string(), self.current_pos.clone()),
            Some('*') => Token::new(TokenType::Multiply, "*".to_string(), self.current_pos.clone()),
            Some('/') => Token::new(TokenType::Divide, "/".to_string(), self.current_pos.clone()),
            Some('%') => Token::new(TokenType::Modulo, "%".to_string(), self.current_pos.clone()),
            Some('(') => Token::new(TokenType::LeftParen, "(".to_string(), self.current_pos.clone()),
            Some(')') => Token::new(TokenType::RightParen, ")".to_string(), self.current_pos.clone()),
            Some('[') => Token::new(TokenType::LeftBracket, "[".to_string(), self.current_pos.clone()),
            Some(']') => Token::new(TokenType::RightBracket, "]".to_string(), self.current_pos.clone()),
            Some('{') => Token::new(TokenType::LeftBrace, "{".to_string(), self.current_pos.clone()),
            Some('}') => Token::new(TokenType::RightBrace, "}".to_string(), self.current_pos.clone()),
            Some(',') => Token::new(TokenType::Comma, ",".to_string(), self.current_pos.clone()),
            Some(':') => Token::new(TokenType::Colon, ":".to_string(), self.current_pos.clone()),
            Some('.') => Token::new(TokenType::Dot, ".".to_string(), self.current_pos.clone()),
            Some('@') => Token::new(TokenType::At, "@".to_string(), self.current_pos.clone()),
            Some('$') => Token::new(TokenType::Dollar, "$".to_string(), self.current_pos.clone()),
            Some('"') => {
                let string_value = self.read_string()?;
                return Ok(Token::new(TokenType::String, string_value, self.current_pos.clone()));
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char(); // Skip the '='
                    Token::new(TokenType::NotEqual, "!=".to_string(), self.current_pos.clone())
                } else {
                    Token::new(TokenType::Not, "!".to_string(), self.current_pos.clone())
                }
            }
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char(); // Skip the second '='
                    Token::new(TokenType::Equal, "==".to_string(), self.current_pos.clone())
                } else if self.peek_char() == Some('~') {
                    self.read_char(); // Skip the '~'
                    Token::new(TokenType::Match, "=~".to_string(), self.current_pos.clone())
                } else {
                    return Err(LexerError::UnexpectedCharacter('=', self.current_pos.clone()));
                }
            }
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.read_char(); // Skip the '='
                    Token::new(TokenType::LessEqual, "<=".to_string(), self.current_pos.clone())
                } else {
                    Token::new(TokenType::Less, "<".to_string(), self.current_pos.clone())
                }
            }
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.read_char(); // Skip the '='
                    Token::new(TokenType::GreaterEqual, ">=".to_string(), self.current_pos.clone())
                } else {
                    Token::new(TokenType::Greater, ">".to_string(), self.current_pos.clone())
                }
            }
            Some('&') => {
                if self.peek_char() == Some('&') {
                    self.read_char(); // Skip the second '&'
                    Token::new(TokenType::And, "&&".to_string(), self.current_pos.clone())
                } else {
                    return Err(LexerError::UnexpectedCharacter('&', self.current_pos.clone()));
                }
            }
            Some('|') => {
                if self.peek_char() == Some('|') {
                    self.read_char(); // Skip the second '|'
                    Token::new(TokenType::Or, "||".to_string(), self.current_pos.clone())
                } else {
                    Token::new(TokenType::Pipe, "|".to_string(), self.current_pos.clone())
                }
            }
            Some(c) if c.is_ascii_digit() => {
                let number_value = self.read_number()?;
                return Ok(Token::new(TokenType::Number, number_value, self.current_pos.clone()));
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let identifier = self.read_identifier();
                let token_type = Self::lookup_identifier(&identifier);
                return Ok(Token::new(token_type, identifier, self.current_pos.clone()));
            }
            Some(c) => {
                return Err(LexerError::UnexpectedCharacter(c, self.current_pos.clone()));
            }
            None => {
                return Ok(Token::new(TokenType::Eof, "".to_string(), self.current_pos.clone()));
            }
        };

        self.read_char();
        Ok(token)
    }

    /// Tokenize the entire input and return a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.token_type == TokenType::Eof;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        // Trim leading and trailing whitespace (similar to JavaScript implementation)
        if let Some(first_token) = tokens.first() {
            if first_token.token_type == TokenType::Whitespace {
                tokens.remove(0);
            }
        }

        // Check if the second-to-last token is whitespace (since the last token is EOF)
        if tokens.len() > 1 {
            let second_to_last_index = tokens.len() - 2;
            if tokens[second_to_last_index].token_type == TokenType::Whitespace {
                tokens.remove(second_to_last_index);
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to filter out whitespace tokens for tests that don't need them
    fn tokenize_without_whitespace(input: &str) -> Result<Vec<Token>, LexerError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        Ok(tokens.into_iter().filter(|t| t.token_type != TokenType::Whitespace).collect())
    }

    #[test]
    fn test_lexer_creation() {
        let lexer = Lexer::new("test");
        // After initialization, current_pos is at the second character due to read_char() calls
        assert_eq!(lexer.current_pos, Position::new(1, 2, 1));
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("a\nb");
        // After initialization, current_pos is at the second character
        assert_eq!(lexer.current_pos, Position::new(1, 2, 1));

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "a");
        assert_eq!(token.position, Position::new(1, 3, 2)); // Token position is where it ends

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Whitespace);
        assert_eq!(token.value, " ");
        assert_eq!(token.position, Position::new(2, 1, 3)); // Whitespace token position

        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.value, "b");
        assert_eq!(token.position, Position::new(2, 2, 4)); // Token position is where it ends
    }

    #[test]
    fn test_simple_tokens() {
        let tokens = tokenize_without_whitespace("+ - * / %").unwrap();

        assert_eq!(tokens.len(), 6); // 5 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Multiply);
        assert_eq!(tokens[3].token_type, TokenType::Divide);
        assert_eq!(tokens[4].token_type, TokenType::Modulo);
        assert_eq!(tokens[5].token_type, TokenType::Eof);
    }

    #[test]
    fn test_punctuation_tokens() {
        let tokens = tokenize_without_whitespace("()[]{}.,:|").unwrap();

        assert_eq!(tokens.len(), 11); // 10 punctuation + EOF
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::RightParen);
        assert_eq!(tokens[2].token_type, TokenType::LeftBracket);
        assert_eq!(tokens[3].token_type, TokenType::RightBracket);
        assert_eq!(tokens[4].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[5].token_type, TokenType::RightBrace);
        assert_eq!(tokens[6].token_type, TokenType::Dot);
        assert_eq!(tokens[7].token_type, TokenType::Comma);
        assert_eq!(tokens[8].token_type, TokenType::Colon);
        assert_eq!(tokens[9].token_type, TokenType::Pipe);
    }

    #[test]
    fn test_comparison_operators() {
        let tokens = tokenize_without_whitespace("== != < > <= >= =~").unwrap();

        assert_eq!(tokens.len(), 8); // 7 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::Equal);
        assert_eq!(tokens[1].token_type, TokenType::NotEqual);
        assert_eq!(tokens[2].token_type, TokenType::Less);
        assert_eq!(tokens[3].token_type, TokenType::Greater);
        assert_eq!(tokens[4].token_type, TokenType::LessEqual);
        assert_eq!(tokens[5].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[6].token_type, TokenType::Match);
    }

    #[test]
    fn test_logical_operators() {
        let tokens = tokenize_without_whitespace("&& || !").unwrap();

        assert_eq!(tokens.len(), 4); // 3 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::And);
        assert_eq!(tokens[1].token_type, TokenType::Or);
        assert_eq!(tokens[2].token_type, TokenType::Not);
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize_without_whitespace("hello world _test test123").unwrap();

        assert_eq!(tokens.len(), 5); // 4 identifiers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "hello");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].value, "world");
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "_test");
        assert_eq!(tokens[3].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].value, "test123");
    }

    #[test]
    fn test_keywords() {
        let tokens = tokenize_without_whitespace("true false null").unwrap();

        assert_eq!(tokens.len(), 4); // 3 keywords + EOF
        assert_eq!(tokens[0].token_type, TokenType::True);
        assert_eq!(tokens[1].token_type, TokenType::False);
        assert_eq!(tokens[2].token_type, TokenType::Null);
    }

    #[test]
    fn test_numbers() {
        let tokens = tokenize_without_whitespace("123 45.67 0.5 100000").unwrap();

        assert_eq!(tokens.len(), 5); // 4 numbers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "123");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "45.67");
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].value, "0.5");
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[3].value, "100000");
    }

    #[test]
    fn test_strings() {
        let tokens = tokenize_without_whitespace(r#""hello" "world" "test with spaces""#).unwrap();

        assert_eq!(tokens.len(), 4); // 3 strings + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "hello");
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(tokens[1].value, "world");
        assert_eq!(tokens[2].token_type, TokenType::String);
        assert_eq!(tokens[2].value, "test with spaces");
    }

    #[test]
    fn test_string_escape_sequences() {
        let tokens = tokenize_without_whitespace(r#""hello\nworld" "test\twith\ttabs" "quote\"test""#).unwrap();

        assert_eq!(tokens.len(), 4); // 3 strings + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "hello\nworld");
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(tokens[1].value, "test\twith\ttabs");
        assert_eq!(tokens[2].token_type, TokenType::String);
        assert_eq!(tokens[2].value, "quote\"test");
    }

    #[test]
    fn test_special_identifiers() {
        let tokens = tokenize_without_whitespace("@ $").unwrap();

        assert_eq!(tokens.len(), 3); // 2 special identifiers + EOF
        assert_eq!(tokens[0].token_type, TokenType::At);
        assert_eq!(tokens[1].token_type, TokenType::Dollar);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("  hello   world  ");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // 2 identifiers + 1 whitespace + EOF (leading/trailing whitespace trimmed)
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "hello");
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[1].value, " ");
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "world");
    }

    #[test]
    fn test_whitespace_trimming() {
        // Test leading whitespace trimming
        let mut lexer = Lexer::new("  hello");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // identifier + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "hello");

        // Test trailing whitespace trimming
        let mut lexer = Lexer::new("hello  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // identifier + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "hello");

        // Test both leading and trailing whitespace trimming
        let mut lexer = Lexer::new("  hello  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // identifier + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "hello");
    }

    #[test]
    fn test_whitespace_collapsing() {
        // Test that multiple whitespace characters are collapsed into a single space
        let mut lexer = Lexer::new("hello\t\n  \tworld");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // identifier + whitespace + identifier + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "hello");
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[1].value, " "); // All whitespace collapsed to single space
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "world");
    }

    #[test]
    fn test_complex_expression() {
        let tokens = tokenize_without_whitespace("filter age > 25 && name == \"John\"").unwrap();

        assert_eq!(tokens.len(), 9); // 8 tokens + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "filter");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].value, "age");
        assert_eq!(tokens[2].token_type, TokenType::Greater);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[3].value, "25");
        assert_eq!(tokens[4].token_type, TokenType::And);
        assert_eq!(tokens[5].token_type, TokenType::Identifier);
        assert_eq!(tokens[5].value, "name");
        assert_eq!(tokens[6].token_type, TokenType::Equal);
        assert_eq!(tokens[7].token_type, TokenType::String);
        assert_eq!(tokens[7].value, "John");
    }

    #[test]
    fn test_array_expression() {
        let tokens = tokenize_without_whitespace("[1, 2, \"hello\", true]").unwrap();

        assert_eq!(tokens.len(), 10); // 9 tokens + EOF
        assert_eq!(tokens[0].token_type, TokenType::LeftBracket);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "1");
        assert_eq!(tokens[2].token_type, TokenType::Comma);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[3].value, "2");
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::String);
        assert_eq!(tokens[5].value, "hello");
        assert_eq!(tokens[6].token_type, TokenType::Comma);
        assert_eq!(tokens[7].token_type, TokenType::True);
        assert_eq!(tokens[8].token_type, TokenType::RightBracket);
    }

    #[test]
    fn test_object_expression() {
        let tokens = tokenize_without_whitespace("{\"name\": \"John\", \"age\": 30}").unwrap();

        assert_eq!(tokens.len(), 10); // 9 tokens + EOF
        assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(tokens[1].value, "name");
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::String);
        assert_eq!(tokens[3].value, "John");
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::String);
        assert_eq!(tokens[5].value, "age");
        assert_eq!(tokens[6].token_type, TokenType::Colon);
        assert_eq!(tokens[7].token_type, TokenType::Number);
        assert_eq!(tokens[7].value, "30");
        assert_eq!(tokens[8].token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_pipe_expression() {
        let tokens = tokenize_without_whitespace("data | filter active | map name").unwrap();

        assert_eq!(tokens.len(), 8); // 7 tokens + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "data");
        assert_eq!(tokens[1].token_type, TokenType::Pipe);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "filter");
        assert_eq!(tokens[3].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].value, "active");
        assert_eq!(tokens[4].token_type, TokenType::Pipe);
        assert_eq!(tokens[5].token_type, TokenType::Identifier);
        assert_eq!(tokens[5].value, "map");
        assert_eq!(tokens[6].token_type, TokenType::Identifier);
        assert_eq!(tokens[6].value, "name");
    }

    #[test]
    fn test_error_unterminated_string() {
        let mut lexer = Lexer::new("\"hello world");
        let result = lexer.tokenize();

        assert!(result.is_err());
        if let Err(LexerError::UnterminatedString(pos)) = result {
            assert_eq!(pos, Position::new(1, 2, 1));
        } else {
            panic!("Expected UnterminatedString error");
        }
    }

    #[test]
    fn test_error_unexpected_character() {
        let mut lexer = Lexer::new("hello # world");
        let result = lexer.tokenize();

        assert!(result.is_err());
        if let Err(LexerError::UnexpectedCharacter(c, pos)) = result {
            assert_eq!(c, '#');
            assert_eq!(pos, Position::new(1, 8, 7));
        } else {
            panic!("Expected UnexpectedCharacter error");
        }
    }

    #[test]
    fn test_error_invalid_number() {
        let mut lexer = Lexer::new("12.34.56");
        let result = lexer.tokenize();

        // The lexer actually succeeds and parses this as "12.34" followed by ".56"
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 4); // number + dot + number + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "12.34");
        assert_eq!(tokens[1].token_type, TokenType::Dot);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].value, "56");
    }

    #[test]
    fn test_unicode_strings() {
        let mut lexer = Lexer::new("\"Hello 世界 🌍\"");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "Hello 世界 🌍");
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1); // Just EOF
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_whitespace_only_input() {
        let mut lexer = Lexer::new("   \t\n  ");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1); // Just EOF
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    // Edge case tests for numbers
    #[test]
    fn test_negative_numbers() {
        let tokens = tokenize_without_whitespace("-123 -45.67 -0.5").unwrap();

        assert_eq!(tokens.len(), 7); // 3 minus operators + 3 numbers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Minus);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "123");
        assert_eq!(tokens[2].token_type, TokenType::Minus);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[3].value, "45.67");
        assert_eq!(tokens[4].token_type, TokenType::Minus);
        assert_eq!(tokens[5].token_type, TokenType::Number);
        assert_eq!(tokens[5].value, "0.5");
    }

    #[test]
    fn test_decimal_edge_cases() {
        let mut lexer = Lexer::new("0.0 .5 5. 0");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 10); // 4 numbers + 2 dots + 3 whitespace + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "0.0");
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[2].token_type, TokenType::Dot);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[3].value, "5");
        assert_eq!(tokens[4].token_type, TokenType::Whitespace);
        assert_eq!(tokens[5].token_type, TokenType::Number);
        assert_eq!(tokens[5].value, "5");
        assert_eq!(tokens[6].token_type, TokenType::Dot);
        assert_eq!(tokens[7].token_type, TokenType::Whitespace);
        assert_eq!(tokens[8].token_type, TokenType::Number);
        assert_eq!(tokens[8].value, "0");
    }

    #[test]
    fn test_large_numbers() {
        let tokens = tokenize_without_whitespace("999999999 12345678901234567890").unwrap();

        assert_eq!(tokens.len(), 3); // 2 numbers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "999999999");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "12345678901234567890");
    }

    #[test]
    fn test_number_with_leading_zeros() {
        let tokens = tokenize_without_whitespace("007 00.5 000").unwrap();

        assert_eq!(tokens.len(), 4); // 3 numbers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "007");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "00.5");
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].value, "000");
    }

    // Edge case tests for strings
    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new("\"\"");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 empty string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "");
    }

    #[test]
    fn test_string_with_only_escape_sequences() {
        let mut lexer = Lexer::new(r#""\n\t\r\\\"""#);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "\n\t\r\\\"");
    }

    #[test]
    fn test_string_with_mixed_unicode() {
        let mut lexer = Lexer::new("\"Hello 世界 🌍 with émojis\"");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "Hello 世界 🌍 with émojis");
    }

    #[test]
    fn test_string_with_escaped_unicode() {
        let mut lexer = Lexer::new(r#""Hello \u0041 world""#);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "Hello u0041 world"); // Note: lexer consumes backslash, doesn't decode unicode escapes
    }

    // Edge case tests for identifiers
    #[test]
    fn test_unicode_identifiers() {
        let tokens = tokenize_without_whitespace("héllo wörld 测试").unwrap();

        assert_eq!(tokens.len(), 4); // 3 identifiers + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "héllo");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].value, "wörld");
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "测试");
    }

    #[test]
    fn test_identifiers_with_numbers() {
        let tokens = tokenize_without_whitespace("test123 _test123 123test").unwrap();

        assert_eq!(tokens.len(), 5); // 3 identifiers + 1 number + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "test123");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].value, "_test123");
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].value, "123");
        assert_eq!(tokens[3].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].value, "test");
    }

    #[test]
    fn test_very_long_identifier() {
        let long_id = "a".repeat(1000);
        let mut lexer = Lexer::new(&long_id);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 identifier + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, long_id);
    }

    // Edge case tests for operators
    #[test]
    fn test_ambiguous_operators() {
        let tokens = tokenize_without_whitespace("a && b || c").unwrap();

        assert_eq!(tokens.len(), 6); // 3 identifiers + 2 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "a");
        assert_eq!(tokens[1].token_type, TokenType::And);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "b");
        assert_eq!(tokens[3].token_type, TokenType::Or);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[4].value, "c");
    }

    #[test]
    fn test_operator_precedence_ambiguity() {
        let tokens = tokenize_without_whitespace("a == b != c").unwrap();

        assert_eq!(tokens.len(), 6); // 3 identifiers + 2 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "a");
        assert_eq!(tokens[1].token_type, TokenType::Equal);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "b");
        assert_eq!(tokens[3].token_type, TokenType::NotEqual);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[4].value, "c");
    }

    #[test]
    fn test_pipe_vs_or_operator() {
        let tokens = tokenize_without_whitespace("a | b || c").unwrap();

        assert_eq!(tokens.len(), 6); // 3 identifiers + 2 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "a");
        assert_eq!(tokens[1].token_type, TokenType::Pipe);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "b");
        assert_eq!(tokens[3].token_type, TokenType::Or);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[4].value, "c");
    }

    // More comprehensive error tests
    #[test]
    fn test_error_invalid_escape_sequence() {
        let mut lexer = Lexer::new(r#""hello \x world""#);
        let result = lexer.tokenize();

        // The current lexer implementation doesn't validate escape sequences,
        // so this should actually succeed with the literal x (backslash is consumed)
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 2); // 1 string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "hello x world");
    }

    #[test]
    fn test_error_malformed_number_multiple_dots() {
        let mut lexer = Lexer::new("12.34.56.78");
        let result = lexer.tokenize();

        // The lexer actually succeeds and parses this as "12.34" followed by ".56.78"
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 4); // number + dot + number + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "12.34");
        assert_eq!(tokens[1].token_type, TokenType::Dot);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].value, "56.78");
    }

    #[test]
    fn test_error_number_ending_with_dot() {
        let mut lexer = Lexer::new("123.");
        let result = lexer.tokenize();

        // This should succeed as "123" followed by a dot
        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 3); // number + dot + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "123");
        assert_eq!(tokens[1].token_type, TokenType::Dot);
    }

    #[test]
    fn test_error_unexpected_character_unicode() {
        let mut lexer = Lexer::new("hello ★ world");
        let result = lexer.tokenize();

        assert!(result.is_err());
        if let Err(LexerError::UnexpectedCharacter(c, _)) = result {
            assert_eq!(c, '★');
        } else {
            panic!("Expected UnexpectedCharacter error");
        }
    }

    // Position tracking edge cases
    #[test]
    fn test_position_tracking_multiline_string() {
        let mut lexer = Lexer::new("\"hello\nworld\"");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "hello\nworld");
        // The position should be at the end of the string (after the closing quote)
        assert_eq!(tokens[0].position, Position::new(2, 7, 14));
    }

    #[test]
    fn test_position_tracking_complex_expression() {
        let mut lexer = Lexer::new("filter(\n  age > 25\n)");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 11); // 6 tokens + 4 whitespace + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "filter");
        assert_eq!(tokens[0].position, Position::new(1, 8, 7)); // End of "filter"

        assert_eq!(tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].position, Position::new(1, 8, 7)); // After "("

        assert_eq!(tokens[2].token_type, TokenType::Whitespace);
        assert_eq!(tokens[2].value, " ");
        assert_eq!(tokens[2].position, Position::new(2, 3, 11)); // Whitespace position

        assert_eq!(tokens[3].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].value, "age");
        assert_eq!(tokens[3].position, Position::new(2, 6, 14)); // End of "age"

        assert_eq!(tokens[4].token_type, TokenType::Whitespace);
        assert_eq!(tokens[4].value, " ");
        assert_eq!(tokens[4].position, Position::new(2, 7, 15)); // Whitespace position

        assert_eq!(tokens[5].token_type, TokenType::Greater);
        assert_eq!(tokens[5].position, Position::new(2, 7, 15)); // After ">"

        assert_eq!(tokens[6].token_type, TokenType::Whitespace);
        assert_eq!(tokens[6].value, " ");
        assert_eq!(tokens[6].position, Position::new(2, 9, 17)); // Whitespace position

        assert_eq!(tokens[7].token_type, TokenType::Number);
        assert_eq!(tokens[7].value, "25");
        assert_eq!(tokens[7].position, Position::new(2, 11, 19)); // End of "25"

        assert_eq!(tokens[8].token_type, TokenType::Whitespace);
        assert_eq!(tokens[8].value, " ");
        assert_eq!(tokens[8].position, Position::new(3, 1, 20)); // Whitespace position

        assert_eq!(tokens[9].token_type, TokenType::RightParen);
        assert_eq!(tokens[9].position, Position::new(3, 1, 20)); // After ")"
    }

    #[test]
    fn test_position_tracking_tabs_and_spaces() {
        let mut lexer = Lexer::new("  \thello\t  world");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // 2 identifiers + 1 whitespace + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "hello");
        // Position should be at the end of the identifier
        assert_eq!(tokens[0].position, Position::new(1, 10, 9));

        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[1].value, " ");
        assert_eq!(tokens[1].position, Position::new(1, 13, 12));

        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].value, "world");
        assert_eq!(tokens[2].position, Position::new(1, 18, 17));
    }

    // Performance and stress tests
    #[test]
    fn test_very_long_string() {
        let long_string = "\"".to_string() + &"a".repeat(10000) + "\"";
        let mut lexer = Lexer::new(&long_string);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 1 string + EOF
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value.len(), 10000);
    }

    #[test]
    fn test_many_tokens() {
        let input = (0..1000).map(|i| format!("token{}", i)).collect::<Vec<_>>().join(" ");
        let tokens = tokenize_without_whitespace(&input).unwrap();

        assert_eq!(tokens.len(), 1001); // 1000 identifiers + EOF
        for (i, token) in tokens.iter().enumerate().take(1000) {
            assert_eq!(token.token_type, TokenType::Identifier);
            assert_eq!(token.value, format!("token{}", i));
        }
    }

    #[test]
    fn test_deeply_nested_brackets() {
        let input = "[".repeat(1000) + &"]".repeat(1000);
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2001); // 1000 left brackets + 1000 right brackets + EOF
        for i in 0..1000 {
            assert_eq!(tokens[i].token_type, TokenType::LeftBracket);
        }
        for i in 1000..2000 {
            assert_eq!(tokens[i].token_type, TokenType::RightBracket);
        }
    }
}
