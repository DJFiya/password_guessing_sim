//Handle user specified filter configurations for faster solve time.

use crate::utils::get_charset;

pub struct Config {
    pub min_length: usize,
    pub max_length: usize,
    pub charset: Vec<char>,
    pub required_chars: Vec<char>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            min_length: 1,
            max_length: 4,
            charset: get_charset(),
            required_chars: vec!['@', 'D'],
            prefix: Some("D".to_string()),
            suffix: None,
        }
    }
}
