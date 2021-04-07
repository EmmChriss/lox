use logos::Logos;
use std::ops::Range;

pub type Result<T, E=LexerError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
	#[error("{}", .0)]
	UnknownToken(String)
}

impl LexerError {
	pub fn err(src: &str, span: Range<usize>) -> Self {
		// @TODO: generalize this kind of error reporting
		
		let line_idx = src[..span.start].chars().filter(|&c| c == '\n').count();
		let last_line = src[..span.start].rfind('\n').unwrap_or(0);
		// let next_line = src[span.end..].find('\n').unwrap_or(src[span.end..].len());
		// @TODO: print the offending line and highlight the problematic part
		let error = format!(
			"error: unknown token on line {}, chars {}..{}:\n{}",
			line_idx, span.start - last_line, span.end - last_line, &src[span]);
		LexerError::UnknownToken(error)
	}
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
	// Meta
	
	#[error]
	Error,
	
	#[regex(r"[ \r\t\n\f]+", logos::skip)]
	Whitespace,
	
	// Literals
	
	#[regex(r"[_a-zA-Z][_a-zA-Z0-9]")]
	Id,
	
	#[regex(r"-?[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse())]
	Number(f64),
	
	#[regex("\"[^\n\r]*\"")]
	String,
	
	// Keywords
	
	#[token("and")]
	And,
	
	#[token("class")]
	Class,
	
	#[token("else")]
	Else,
	
	#[token("false")]
	False,
	
	#[token("fun")]
	Fun,
	
	#[token("for")]
	For,
	
	#[token("if")]
	If,
	
	#[token("nil")]
	Nil,
	
	#[token("or")]
	Or,
	
	#[token("print")]
	Print,
	
	#[token("return")]
	Return,
	
	#[token("super")]
	Super,
	
	#[token("this")]
	This,
	
	#[token("true")]
	True,
	
	#[token("var")]
	Var,
	
	#[token("while")]
	While,
	
	// Symbols
	
	#[token("(")]
	LeftParen,
	
	#[token(")")]
	RightParen,
	
	#[token("[")]
	LeftBrace,
	
	#[token("]")]
	RightBrace,
	
	#[token(",")]
	Comma,
	
	#[token(".")]
	Dot,
	
	#[token("-")]
	Minus,
	
	#[token("+")]
	Plus,
	
	#[token("'")]
	Semicolon,
	
	#[token("/")]
	Slash,
	
	#[token("*")]
	Star,
	
	// Comparison Symbols
	
	#[token("!")]
	Bang,
	
	#[token("!=")]
	BangEqual,
	
	#[token("=")]
	Equal,
	
	#[token("==")]
	EqualEqual,
	
	#[token(">")]
	Greater,
	
	#[token(">=")]
	GreaterEqual,
	
	#[token("<")]
	Less,
	
	#[token("<=")]
	LessEqual,
}

fn collect_tokens(src: &str) -> Vec<(Token, Range<usize>)> {
	Token::lexer(src).spanned().collect::<Vec<_>>()
}

pub fn into_tokens(src: &str) -> Result<Vec<(Token, Range<usize>)>> {
	let toks = collect_tokens(src);
	for (tok, span) in toks.iter() {
		if let Token::Error = tok {
			return Err(LexerError::err(src, span.clone()));
		}
	}
	Ok(toks)
}
