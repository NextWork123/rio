use crate::sugarloaf::constants::{create_sugar_line, LINE_MAX_CHARACTERS};
use crate::sugarloaf::graphics::SugarGraphic;
use serde::Deserialize;
use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct Sugar {
    pub content: char,
    pub repeated: usize,
    pub foreground_color: [f32; 4],
    pub background_color: [f32; 4],
    pub style: SugarStyle,
    pub decoration: SugarDecoration,
    pub cursor: SugarCursor,
    pub media: Option<SugarGraphic>,
}

impl Default for Sugar {
    fn default() -> Self {
        Self {
            content: ' ',
            repeated: 0,
            foreground_color: [0., 0., 0., 0.],
            background_color: [0., 0., 0., 0.],
            style: SugarStyle::default(),
            decoration: SugarDecoration::default(),
            cursor: SugarCursor::default(),
            media: None,
        }
    }
}

impl PartialEq for Sugar {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
            && self.repeated == other.repeated
            && self.foreground_color == other.foreground_color
            && self.background_color == other.background_color
            && self.style == other.style
            && self.decoration == other.decoration
            && self.cursor == other.cursor
    }
}

#[inline]
fn equal_without_consider_repeat(sugar_a: &Sugar, sugar_b: &Sugar) -> bool {
    sugar_a.content == sugar_b.content
        && sugar_a.foreground_color == sugar_b.foreground_color
        && sugar_a.background_color == sugar_b.background_color
        && sugar_a.style == sugar_b.style
        && sugar_a.decoration == sugar_b.decoration
        && sugar_a.cursor == sugar_b.cursor
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum SugarCursor {
    Block([f32; 4]),
    Caret([f32; 4]),
    Underline([f32; 4]),
    #[default]
    Disabled,
}

#[derive(Debug, Copy, PartialEq, Default, Clone)]
pub enum SugarDecoration {
    Underline,
    Strikethrough,
    #[default]
    Disabled,
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct SugarStyle {
    pub is_italic: bool,
    pub is_bold: bool,
    pub is_bold_italic: bool,
}

#[derive(Copy, PartialEq, Default, Debug, Clone)]
pub struct SugarloafStyle {
    pub screen_position: (f32, f32),
    pub line_height: f32,
    pub text_scale: f32,
}

#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub struct ImageProperties {
    #[serde(default = "String::default")]
    pub path: String,
    #[serde(default = "f32::default")]
    pub width: f32,
    #[serde(default = "f32::default")]
    pub height: f32,
    #[serde(default = "f32::default")]
    pub x: f32,
    #[serde(default = "f32::default")]
    pub y: f32,
}

/// Contains a line representation that is hashable and comparable
#[derive(Debug, Copy, Clone)]
pub struct SugarLine {
    // hash: u64,
    // Sized arrays can take up to half of time to execute
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b3face22f8c64b25803fa213be6a858f
    inner: [Sugar; LINE_MAX_CHARACTERS],
    pub len: usize,
    pub acc: usize,
    first_non_default: usize,
    last_non_default: usize,
    non_default_count: usize,
    default_sugar: Sugar,
}

impl PartialEq for SugarLine {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.is_empty() && other.is_empty() {
            return true;
        }

        if self.len != other.len
            || self.first_non_default != other.first_non_default
            || self.last_non_default != other.last_non_default
            || self.non_default_count != other.non_default_count
        {
            return false;
        }

        for i in 0..self.len {
            if self.inner[i] != other.inner[i] {
                return false;
            }
        }

        true
    }
}

impl Default for SugarLine {
    fn default() -> Self {
        Self {
            // hash: 00000000000000,
            last_non_default: 0,
            first_non_default: 0,
            non_default_count: 0,
            inner: create_sugar_line(),
            default_sugar: Sugar::default(),
            acc: 0,
            len: 0,
        }
    }
}

impl SugarLine {
    #[inline]
    pub fn insert(&mut self, sugar: Sugar) {
        let previous = if self.acc > 0 { self.acc - 1 } else { 0 };

        if equal_without_consider_repeat(&self.inner[previous], &sugar) {
            self.inner[previous].repeated += 1;
            self.len += 1;
            return;
        }

        self.inner[self.acc] = sugar;

        if sugar != self.default_sugar {
            if self.first_non_default == 0 {
                self.first_non_default = self.acc;
                self.last_non_default = self.acc;
            } else {
                self.last_non_default = self.acc;
            }

            self.non_default_count += 1;
        }

        self.acc += 1;
        self.len += 1;
    }

    #[inline]
    pub fn insert_empty(&mut self) {
        self.inner[self.len] = self.default_sugar;
        self.acc += 1;
        self.len += 1;
    }

    // #[inline]
    // fn compute_hash(&mut self) {
    // 00000000000000
    // 00000000000000 -> first non-default apparison position
    // 00000000000000 -> last non-default apparison position
    // 00000000000000 ->
    // }

    #[inline]
    pub fn is_empty(&self) -> bool {
        // if first digits are zero
        self.non_default_count == 0
    }

    #[inline]
    pub fn from_vec(&mut self, vector: &Vec<Sugar>) {
        for element in vector.into_iter() {
            self.insert(*element)
        }
    }
}

impl Index<usize> for SugarLine {
    type Output = Sugar;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sugarline_comparisson_exact_match() {
        let line_a = SugarLine::default();
        let line_b = SugarLine::default();

        assert!(line_a.is_empty());
        assert!(line_b.is_empty());
        assert_eq!(line_a, line_b);
    }

    #[test]
    fn test_sugarline_from_vector() {
        let mut line_a = SugarLine::default();
        let vector = vec![
            Sugar {
                content: 't',
                ..Sugar::default()
            },
            Sugar {
                content: 'e',
                ..Sugar::default()
            },
            Sugar {
                content: 'r',
                ..Sugar::default()
            },
            Sugar {
                content: 'm',
                ..Sugar::default()
            },
        ];

        line_a.from_vec(&vector);

        assert!(!line_a.is_empty());
        assert_eq!(line_a.len, 4);
    }

    #[test]
    fn test_sugarline_repetition() {
        let mut line_a = SugarLine::default();
        let vector = vec![
            Sugar {
                content: 'a',
                ..Sugar::default()
            },
            Sugar {
                content: 'a',
                ..Sugar::default()
            },
            Sugar {
                content: 'b',
                ..Sugar::default()
            },
            Sugar {
                content: 'c',
                ..Sugar::default()
            },
            Sugar {
                content: 'd',
                ..Sugar::default()
            },
            Sugar {
                content: 'd',
                ..Sugar::default()
            },
        ];

        line_a.from_vec(&vector);

        assert!(!line_a.is_empty());
        assert_eq!(line_a.len, 6);
        assert_eq!(line_a.acc, 4);

        let mut line_a = SugarLine::default();
        let vector = vec![
            Sugar {
                content: 'a',
                ..Sugar::default()
            },
            Sugar {
                content: 'b',
                ..Sugar::default()
            },
            Sugar {
                content: 'c',
                ..Sugar::default()
            },
            Sugar {
                content: 'd',
                ..Sugar::default()
            },
            Sugar {
                content: 'e',
                ..Sugar::default()
            },
            Sugar {
                content: 'f',
                ..Sugar::default()
            },
        ];

        line_a.from_vec(&vector);

        assert!(!line_a.is_empty());
        assert_eq!(line_a.len, 6);
        assert_eq!(line_a.acc, 6);

        let mut line_a = SugarLine::default();
        let vector = vec![
            Sugar {
                content: ' ',
                ..Sugar::default()
            },
            Sugar {
                content: ' ',
                ..Sugar::default()
            },
            Sugar {
                content: ' ',
                ..Sugar::default()
            },
            Sugar {
                content: ' ',
                ..Sugar::default()
            },
            Sugar {
                content: ' ',
                ..Sugar::default()
            },
            Sugar {
                content: ' ',
                ..Sugar::default()
            },
        ];

        line_a.from_vec(&vector);

        assert!(line_a.is_empty());
        assert_eq!(line_a.len, 6);
        assert_eq!(line_a.acc, 0);
    }

    #[test]
    fn test_sugarline_empty_checks() {
        let mut line_a = SugarLine::default();
        line_a.insert_empty();
        line_a.insert_empty();
        line_a.insert_empty();

        assert!(line_a.is_empty());

        let mut line_a = SugarLine::default();
        line_a.insert(Sugar::default());

        assert!(line_a.is_empty());

        let mut line_a = SugarLine::default();
        line_a.insert(Sugar {
            content: ' ',
            ..Sugar::default()
        });

        assert!(line_a.is_empty());
    }

    #[test]
    fn test_sugarline_comparisson_different_len() {
        let mut line_a = SugarLine::default();
        line_a.insert_empty();
        line_a.insert(Sugar {
            content: 'r',
            ..Sugar::default()
        });
        let line_b = SugarLine::default();

        assert!(!line_a.is_empty());
        assert!(line_b.is_empty());
        assert!(line_a != line_b);

        let mut line_a = SugarLine::default();
        line_a.insert(Sugar {
            content: ' ',
            ..Sugar::default()
        });
        line_a.insert(Sugar {
            content: 'r',
            ..Sugar::default()
        });
        let mut line_b = SugarLine::default();
        line_b.insert(Sugar {
            content: 'r',
            ..Sugar::default()
        });
        line_b.insert(Sugar {
            content: ' ',
            ..Sugar::default()
        });
        line_b.insert(Sugar {
            content: 'i',
            ..Sugar::default()
        });
        line_b.insert(Sugar {
            content: 'o',
            ..Sugar::default()
        });

        assert!(!line_a.is_empty());
        assert!(!line_b.is_empty());
        assert!(line_a != line_b);
    }

    #[test]
    fn test_sugarline_comparisson_different_match_with_same_len() {
        let mut line_a = SugarLine::default();
        line_a.insert(Sugar {
            content: 'o',
            ..Sugar::default()
        });
        line_a.insert(Sugar {
            content: 'i',
            ..Sugar::default()
        });
        line_a.insert(Sugar {
            content: 'r',
            ..Sugar::default()
        });
        let mut line_b = SugarLine::default();
        line_b.insert(Sugar {
            content: 'r',
            ..Sugar::default()
        });
        line_b.insert(Sugar {
            content: 'i',
            ..Sugar::default()
        });
        line_b.insert(Sugar {
            content: 'o',
            ..Sugar::default()
        });

        assert!(!line_a.is_empty());
        assert!(!line_b.is_empty());
        assert!(line_a != line_b);
    }
}
