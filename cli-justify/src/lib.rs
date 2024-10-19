fn split_at_char(s: &str, n: usize) -> (&str, Option<&str>) {
  let mut char_index = 0;

  for (i, _) in s.char_indices() {
    if char_index == n {
      let (w1, w2) = s.split_at(i);
      return (w1, Some(w2));
    }
    char_index += 1;
  }

  (s, None)
}

pub fn justify(text: &str, line_width: usize) -> Vec<String> {
  let paragraphs: Vec<&str> = text.split("\n\n").collect();
  let mut lines: Vec<String> = Vec::new();

  for paragraph in paragraphs {
    let raw_words: Vec<&str> = paragraph.split_whitespace().collect();
    let mut words = vec![];

    for mut word in raw_words {
      while let (w1, Some(w2)) = split_at_char(word, line_width) {
        words.push(w1);
        word = w2;
      }

      words.push(word);
    }

    let mut line: Vec<&str> = Vec::new();
    let mut len = 0;

    for word in words {
      if len + word.len() > line_width {
        lines.push(justify_line(&line, line_width));
        line.clear();
        len = 0;
      }
      line.push(word);
      len += word.len() + 1; // +1 for space
    }

    // Add the last line of the paragraph
    if !line.is_empty() {
      lines.push(line.join(" "));
    }

    // Add a blank line after each paragraph to preserve paragraph breaks
    lines.push(String::new());
  }

  lines
}

fn justify_line(line: &[&str], line_width: usize) -> String {
  let word_len: usize = line.iter().map(|s| s.len()).sum();
  let spaces = line_width - word_len;

  let line_len_div = if (line.len() > 1) { (line.len() - 1) } else { 1 };

  let each_space = spaces / line_len_div;
  let extra_space = spaces % line_len_div;

  let mut justified = String::new();
  for (i, word) in line.iter().enumerate() {
    justified.push_str(word);
    if i < line.len() - 1 {
      let mut space = " ".repeat(each_space);
      if i < extra_space {
        space.push(' ');
      }
      justified.push_str(&space);
    }
  }

  justified
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_handles_long_words() {
    let input_text = r#"some text and a very loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong word but no cause to panic"#;
    let pretty_short_line_width = 10;
    justify(input_text, pretty_short_line_width);
  }
}
