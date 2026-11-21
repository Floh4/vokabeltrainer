use crate::error::VocabError;

// TODO: Maybe add an function that takes vocabulary and translation as parameters and returns the struct vocabulary.

pub struct Vocabulary {
    pub(crate) id: u32,
    pub(crate) vocabulary: String,
    pub(crate) translation: String,
    pub(crate) vocab_check_count: i64,
    pub(crate) correct: f32,
    pub(crate) language: String,
    pub(crate) group: Option<String>,
    pub(crate) vocab_info: Option<String>,
    pub(crate) translation_info: Option<String>,
    pub(crate) info: Option<String>,
}

impl Default for Vocabulary {
    fn default() -> Self {
        Vocabulary {
            id: 1, // it is not recommended to use the default values for add_vocab for the columns
            // that have the UNIQUE sql constraint
            vocabulary: "Foo".to_string(),
            translation: "Bar".to_string(),
            vocab_check_count: 1,
            correct: 1.0,
            language: "Rust".to_string(),
            group: None,
            vocab_info: None,
            translation_info: None,
            info: None,
        }
    }
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
            let levenshtein_distance = levenshtein(&self.translation, input);
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

pub fn add(vocabulary: &str, translation: &str) -> Vocabulary {
    Vocabulary {
        vocabulary: vocabulary.to_string(),
        translation: translation.to_string(),
        ..Default::default()
    }
}
