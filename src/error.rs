pub type Result<T, E=LoxError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum LoxError {
	#[error("...")]
	IoError(#[from] std::io::Error),
	
	#[error("lexer error: {:?}", .0)]
	LexerError(#[from] crate::lexer::LexerError)
}
