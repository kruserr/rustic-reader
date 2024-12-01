pub fn get_tutorial_text() -> Vec<String> {
  vec![
    "Welcome to cli-text-reader!".to_string(),
    "".to_string(),
    "Navigation:".to_string(),
    "j or ↓ = scroll down".to_string(),
    "k or ↑ = scroll up".to_string(),
    "PageDown = scroll down one page".to_string(),
    "PageUp = scroll up one page".to_string(),
    "".to_string(),
    "Search:".to_string(),
    "/ = search forward".to_string(),
    "? = search backward".to_string(),
    "n = next match".to_string(),
    "N = previous match".to_string(),
    "".to_string(),
    "Commands:".to_string(),
    ": then type command:".to_string(),
    "q = quit".to_string(),
    "z = toggle line highlighter".to_string(),
    "p = toggle progress".to_string(),
    "help or tutorial = show this tutorial".to_string(),
    "".to_string(),
    "Press any key to continue...".to_string(),
  ]
}