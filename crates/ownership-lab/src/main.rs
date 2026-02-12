// 1. Implement `take_last_note`:
//    - It must take ownership of `Vec<String>`.
//    - Remove the last note from the vector.
//    - Return `(last_note, remaining_notes)`.
fn take_last_note(mut notes: Vec<String>) -> Option<(String, Vec<String>)> {
  notes.pop().map(|last| (last, notes))
}

// 2. Implement `count_keyword`:
//    - Borrow notes immutably.
//    - Count how many notes contain `keyword`.
fn count_keyword(notes: &[String], keyword: &str) -> usize {
    notes.iter().fold(0, |acc, note| {
      if note.contains(keyword) { acc + 1 } else { acc }
    })
}

// 3. Implement `normalize_notes`:
//    - Borrow notes mutably.
//    - Trim whitespace for each note.
//    - Convert each note to lowercase.
fn normalize_notes(notes: &mut [String]) {
    notes.iter_mut().for_each(|note| {
      *note = note.trim().to_lowercase();
    });
}

// 4. Implement `first_word`:
//    - Input: `&str`
//    - Output: `&str` slice containing first word.
//    - If there is no space, return the full string slice.
fn first_word(s: &str) -> &str {
    for (index, value) in s.bytes().enumerate() {
      if value == b' ' { return &s[..index]; }
    }

    s
}

// 5. Longest note
// - Create a `longest_note` function using `&[String] -> Option<&String>`
//   without allocating new strings.
fn longest_note(notes: &[String]) -> Option<&String> {
    notes.iter().max_by_key(|note| note.len())
}

fn main() {
    let notes = vec![
        String::from("  Rust ownership rules  "),
        String::from("Borrowing prevents data races"),
        String::from("Slices are references into data"),
        String::from("  Move semantics transfer ownership  "),
    ];

    let ownership_hits = count_keyword(&notes, "ownership");
    println!("Keyword hits #1: {ownership_hits}");

    let Some((last, mut notes)) = take_last_note(notes) else {
      println!("No notes found");
      return;
    };
    println!("Picked last note: {last}");

    let ownership_hits = count_keyword(&notes, "ownership");
    println!("Keyword hits #2: {ownership_hits}");

    normalize_notes(&mut notes);

    let preview = first_word(&notes[0]);
    println!("First word after normalize: {preview}");

    match longest_note(&notes) {
      Some(max) => println!("Longest note: {:?}", max),
      None => println!("No notes found"),
    };
}
