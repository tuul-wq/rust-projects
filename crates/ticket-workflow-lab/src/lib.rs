#![allow(unused_variables)]

// Topic 1: str slices

use std::str::SplitWhitespace;

// Task 1 (Easy):
// Return a trimmed `&str` slice.
pub fn trim_label(input: &str) -> &str {
  input.trim()
}

// Task 2 (Easy):
// Return the first word from `input`.
// Rules:
// - Ignore leading/trailing spaces.
// - Words are separated by the space character `' '`.
// - Return `\"\"` when there is no word.
pub fn first_word(input: &str) -> &str {
  input.split_whitespace().next().unwrap_or("")
}

// Task 3 (Easy):
// Parse a ticket code in the form `PREFIX-NUMBER`.
// Return `Some((prefix, number))` when valid, otherwise `None`.
// Rules:
// - Prefix length must be 2..=5.
// - Prefix must contain only uppercase ASCII letters.
// - Number must contain only ASCII digits and cannot be empty.
pub fn split_ticket_code(code: &str) -> Option<(&str, &str)> {
  let (prefix, suffix) = code.split_once('-')?;

  let is_valid_length = prefix.len() >= 2 && prefix.len() <= 5;
  let is_uppercase = prefix.is_ascii() && prefix.chars().all(|c| c.is_uppercase());
  let is_valid_suffix = suffix.is_ascii() && !suffix.is_empty();

  if is_valid_length && is_uppercase && is_valid_suffix {
    Some((prefix, suffix))
  } else {
    None
  }
}

// Topic 2: Arrays

// Task 4 (Easy):
// Swap the first and last elements in a fixed-size array.
pub fn swap_ends(values: [i32; 5]) -> [i32; 5] {
  let mut new_array = values;
  new_array.swap(0, 4);

  new_array
}

// Task 5 (Medium):
// Clamp each value to the inclusive range `[min, max]`.
pub fn clamp_scores(values: [i32; 6], min: i32, max: i32) -> [i32; 6] {
    values.map(|v| v.clamp(min, max))
}

// Task 6 (Medium):
// Sum values in the inclusive index range `[start, end]`.
// Return 0 if:
// - `start > end`
// - `end` is out of bounds
pub fn sum_range(values: [i32; 8], start: usize, end: usize) -> i32 {
  if start > end || values.len() < end {
    0
  } else {
    values[start..=end].iter().sum()
  }
}

// Topic 3: Vectors

// Task 7 (Medium):
// Normalize `raw_tag` with `trim()` + lowercase and push it only if:
// - it is not empty
// - it is not already present in `tags`
// Return `true` when pushed, `false` otherwise.
pub fn push_unique_tag(tags: &mut Vec<String>, raw_tag: &str) -> bool {
  let raw_tag = raw_tag.trim().to_lowercase();

  if raw_tag.is_empty() || tags.contains(&raw_tag) {
    return false;
  }

  tags.push(raw_tag);
  true
}

// Task 8 (Medium):
// Remove every tag that becomes empty after `trim()`.
// Return how many tags were removed.
pub fn remove_blank_tags(tags: &mut Vec<String>) -> usize {
    let init_count = tags.len();

    tags.retain(|t| !t.trim().is_empty());

    init_count - tags.len()
}

// Topic 4: Enums + Mixed Practice

pub enum TaskState {
    Todo,
    Doing,
    Blocked,
    Done,
}

// Task 9 (Medium):
// Convert a lowercase state name into `TaskState`.
// Supported names: `todo`, `doing`, `blocked`, `done`.
pub fn parse_state(name: &str) -> Option<TaskState> {
  match name {
    "todo" => Some(TaskState::Todo),
    "doing" => Some(TaskState::Doing),
    "blocked" => Some(TaskState::Blocked),
    "done" => Some(TaskState::Done),
    _ => None,
  }
}

// Task 10 (Advanced):
// Apply one command to `states`.
// Supported commands:
// - `add <state>`
// - `set <index> <state>`
// - `remove <index>`
//
// Rules:
// - `state` uses the same names as Task 9.
// - `index` is zero-based.
// - Whitespace around command should be ignored.
// - Return `Err(String)` for malformed command, unknown state, or invalid index.
pub fn apply_state_command(states: &mut Vec<TaskState>, command: &str) -> Result<(), String> {
  fn malformed() -> String {
    "Malformed data!".to_string()
  }

  fn next_token<'a>(it: &mut SplitWhitespace<'a>) -> Result<&'a str, String> {
    it.next().ok_or_else(malformed)
  }

  fn parse_index(s: &str) -> Result<usize, String> {
    s.parse::<usize>().map_err(|_| malformed())
  }

  fn parse_task_state(s: &str) -> Result<TaskState, String> {
    parse_state(s).ok_or_else(malformed)
  }

  let mut parts = command.trim().split_whitespace();
  let action = next_token(&mut parts)?;

  match action {
    "add" => {
      let state_item = parse_task_state(next_token(&mut parts)?)?;

      states.push(state_item);
      Ok(())
    },
    "set" => {
      let index = parse_index(next_token(&mut parts)?)?;
      let state_item = parse_task_state(next_token(&mut parts)?)?;

      if let Some(slot) = states.get_mut(index) {
        *slot = state_item;
        Ok(())
      } else {
        Err(malformed())
      }
    },
    "remove" => {
      let index = parse_index(next_token(&mut parts)?)?;

      if states.get(index).is_none() {
        return Err(malformed())
      }

      states.remove(index);
      Ok(())
    },
    _ => Err(malformed()),
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_name(state: &TaskState) -> &'static str {
        match state {
            TaskState::Todo => "todo",
            TaskState::Doing => "doing",
            TaskState::Blocked => "blocked",
            TaskState::Done => "done",
        }
    }

    #[test]
    fn task_1_trim_label() {
        assert_eq!(trim_label("  backend  "), "backend");
        assert_eq!(trim_label("core"), "core");
        assert_eq!(trim_label("   "), "");
    }

    #[test]
    fn task_2_first_word() {
        assert_eq!(first_word("  rust slices are fast  "), "rust");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word("    "), "");
    }

    #[test]
    fn task_3_split_ticket_code() {
        assert_eq!(split_ticket_code("BUG-42"), Some(("BUG", "42")));
        assert_eq!(split_ticket_code("AB-0009"), Some(("AB", "0009")));
        assert_eq!(split_ticket_code("aB-77"), None);
        assert_eq!(split_ticket_code("BUG-"), None);
        assert_eq!(split_ticket_code("BUG42"), None);
    }

    #[test]
    fn task_4_swap_ends() {
        assert_eq!(swap_ends([1, 2, 3, 4, 5]), [5, 2, 3, 4, 1]);
    }

    #[test]
    fn task_5_clamp_scores() {
        let values = [12, -3, 7, 99, 4, 0];
        assert_eq!(clamp_scores(values, 0, 10), [10, 0, 7, 10, 4, 0]);
    }

    #[test]
    fn task_6_sum_range() {
        let values = [2, 4, 6, 8, 10, 12, 14, 16];
        assert_eq!(sum_range(values, 2, 4), 24);
        assert_eq!(sum_range(values, 7, 7), 16);
        assert_eq!(sum_range(values, 6, 2), 0);
        assert_eq!(sum_range(values, 0, 20), 0);
    }

    #[test]
    fn task_7_push_unique_tag() {
        let mut tags = vec!["backend".to_string()];

        assert!(!push_unique_tag(&mut tags, "  BACKEND  "));
        assert!(push_unique_tag(&mut tags, "  API  "));
        assert!(!push_unique_tag(&mut tags, "   "));

        assert_eq!(tags, vec!["backend".to_string(), "api".to_string()]);
    }

    #[test]
    fn task_8_remove_blank_tags() {
        let mut tags = vec![
            "bug".to_string(),
            "   ".to_string(),
            "urgent".to_string(),
            "".to_string(),
        ];

        let removed = remove_blank_tags(&mut tags);

        assert_eq!(removed, 2);
        assert_eq!(tags, vec!["bug".to_string(), "urgent".to_string()]);
    }

    #[test]
    fn task_9_parse_state() {
        assert!(matches!(parse_state("todo"), Some(TaskState::Todo)));
        assert!(matches!(parse_state("doing"), Some(TaskState::Doing)));
        assert!(matches!(parse_state("blocked"), Some(TaskState::Blocked)));
        assert!(matches!(parse_state("done"), Some(TaskState::Done)));
        assert!(parse_state("in_progress").is_none());
    }

    #[test]
    fn task_10_apply_state_command() {
        let mut states = vec![TaskState::Todo, TaskState::Blocked];

        apply_state_command(&mut states, " add done ").expect("add should work");
        assert_eq!(states.len(), 3);
        assert_eq!(state_name(&states[2]), "done");

        apply_state_command(&mut states, "set 1 doing").expect("set should work");
        assert_eq!(state_name(&states[1]), "doing");

        apply_state_command(&mut states, "remove 0").expect("remove should work");
        assert_eq!(states.len(), 2);
        assert_eq!(state_name(&states[0]), "doing");

        assert!(apply_state_command(&mut states, "set 20 todo").is_err());
        assert!(apply_state_command(&mut states, "add unknown").is_err());
        assert!(apply_state_command(&mut states, "noop").is_err());
    }
}
