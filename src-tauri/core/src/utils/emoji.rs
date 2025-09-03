use regex::Regex;

/// Represents an emoji parsed from text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedEmoji {
    /// A unicode emoji, stored as its unified string.
    Unicode { unified: String },
    /// A custom guild emoji with optional animation flag.
    Custom {
        id: String,
        name: String,
        animated: bool,
    },
}

/// A part of a parsed string, either raw text or an emoji.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmojiPart {
    Text(String),
    Emoji(ParsedEmoji),
}

/// Parses a string containing Discord style emojis.
///
/// The returned vector alternates between text and emojis, allowing consumers to
/// reconstruct the original string or render custom emoji objects.
pub fn parse_emoji_string(content: &str) -> Vec<EmojiPart> {
    let regex = Regex::new(r"<(a?):(\w+):(\d+)>").expect("invalid regex");
    let mut last_index = 0;
    let mut result = Vec::new();

    for captures in regex.captures_iter(content) {
        if let Some(mat) = captures.get(0) {
            if mat.start() > last_index {
                result.push(EmojiPart::Text(
                    content[last_index..mat.start()].to_string(),
                ));
            }
            let animated = &captures[1] == "a";
            let name = captures[2].to_string();
            let id = captures[3].to_string();
            result.push(EmojiPart::Emoji(ParsedEmoji::Custom { id, name, animated }));
            last_index = mat.end();
        }
    }

    if last_index < content.len() {
        result.push(EmojiPart::Text(content[last_index..].to_string()));
    }

    result
}
