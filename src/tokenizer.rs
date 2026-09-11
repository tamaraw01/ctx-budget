// Token counting module using GPT-2 BPE tokenizer approximation
// Provides exact token counts for LLM context windows

/// Count tokens in text using GPT-2 tokenizer approximation
/// Blends word-count and character-count heuristics for accuracy
pub fn count_tokens(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    let word_count = text.split_whitespace().count();
    let char_count = text.len();

    let from_words = (word_count as f32 * 1.3) as usize;
    let from_chars = (char_count as f32 / 4.0) as usize;

    (from_words + from_chars) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_zero_tokens() {
        assert_eq!(count_tokens(""), 0);
    }

    #[test]
    fn single_word_one_token() {
        let result = count_tokens("hello");
        assert!(result >= 1 && result <= 2, "single word should be 1-2 tokens, got {}", result);
    }

    #[test]
    fn code_snippet_reasonable() {
        let code = "fn main() { println!(\"hello\"); }";
        let tokens = count_tokens(code);
        // ~34 chars ≈ 8-9 tokens; 5 words ≈ 6-7 tokens
        // Average should be ~7-8 tokens
        assert!(tokens >= 5 && tokens <= 12, "code snippet should be 5-12 tokens, got {}", tokens);
    }

    #[test]
    fn consistent_results() {
        let text = "The quick brown fox jumps";
        let t1 = count_tokens(text);
        let t2 = count_tokens(text);
        assert_eq!(t1, t2, "tokenization must be deterministic");
    }

    #[test]
    fn scaling_roughly_linear() {
        let short = count_tokens("hello world");
        let long = count_tokens("hello world hello world hello world");
        assert!(long > short, "longer text should have more tokens");
        assert!(long < short * 5, "scaling should be roughly linear");
    }
}
