use crate::error::VocabError;

pub struct Vocabulary {
    pub(crate) id: u32,
    pub(crate) vocabulary: String,
    pub(crate) translation: String,
    pub(crate) vocab_check_count: i64,
    pub(crate) correct: f32,
}

impl Vocabulary {
    pub fn check(&mut self, input: &str) -> rusqlite::Result<f32, VocabError> {
        use levenshtein::levenshtein;
        // add 1 to VocabCheckCount
        self.vocab_check_count += 1i64;
        let corect: f32;
        // prevent zero division
        if input.len() == 0 {
            corect = 0.0;
        } else {
            let levenshtein_distance = levenshtein(self.translation.as_str(), input);
            if levenshtein_distance == 0 {
                corect = 1.0;
            } else {
                corect =
                    1.0 - (levenshtein_distance as f32 / self.translation.chars().count() as f32);
            }
        }
        self.correct += corect;
        Ok(corect)
    }
}
