//! Provides utilities for implementing Syntax Highlight

/// A interface of syntax highlight
/// This is called just before buffer is displayed.
pub trait Syntaxer: Send + Sync {
    /// syntax highlight buffer
    fn highlight(&self, buf: &str, pos: usize) -> Option<String>;
}

/// It is default syntax highlight
pub struct DummySyntaxer;

impl Syntaxer for DummySyntaxer {
    fn highlight(&self, _buf: &str, _pos: usize) -> Option<String> {
        None
    }
}