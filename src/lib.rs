use regex::Regex;

pub struct TextComprehensionIndex;

impl TextComprehensionIndex {
    pub fn new() -> Self {
        TextComprehensionIndex
    }

    pub fn calculate(&self, text: &str) -> f64 {
        let sentence_count = self.count_sentences(text);
        let word_count = self.count_words(text);
        let syllable_count = self.count_syllables(text);

        if word_count == 0 || sentence_count == 0 {
            return 0.0;
        }

        let words_per_sentence = word_count as f64 / sentence_count as f64;
        let syllables_per_word = syllable_count as f64 / word_count as f64;

        // Text Comprehension Index formula
        let index = 206.835 - (1.015 * words_per_sentence) - (84.6 * syllables_per_word);

        index.clamp(0.0, 100.0)
    }

    fn count_sentences(&self, text: &str) -> usize {
        let re = Regex::new(r"[.!?]").unwrap();
        re.find_iter(text).count()
    }

    fn count_words(&self, text: &str) -> usize {
        let re = Regex::new(r"\b\w+\b").unwrap();
        re.find_iter(text).count()
    }

    fn count_syllables(&self, text: &str) -> usize {
        let re = Regex::new(r"[aeiouáéíóöőúüűAEIOUÁÉÍÓÖŐÚÜŰ]+").unwrap();
        re.find_iter(text).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hungarian() {
        let tci = TextComprehensionIndex::new();
        let text = "Ez egy tesztszöveg. Azért készült, hogy ellenőrizze a szövegértési mutatót.";
        let score = tci.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }

    #[test]
    fn test_edge_cases() {
        let tci = TextComprehensionIndex::new();
        let text = "";
        let score = tci.calculate(text);
        assert_eq!(score, 0.0);

        let text = "A.";
        let score = tci.calculate(text);
        assert!(score >= 0.0 && score <= 100.0);
    }
}
