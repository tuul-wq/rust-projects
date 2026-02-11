/*
Ownership Challenge (Chapter 4): Team Notes Processor

Goal:
Practice ownership, borrowing, mutable references, and string slices
without using `clone()` unless explicitly noted.

Tasks:
1. Implement `take_last_note`:
   - It must take ownership of `Vec<String>`.
   - Remove the last note from the vector.
   - Return `(last_note, remaining_notes)`.

2. Implement `count_keyword`:
   - Borrow notes immutably.
   - Count how many notes contain `keyword`.

3. Implement `normalize_notes`:
   - Borrow notes mutably.
   - Trim whitespace for each note.
   - Convert each note to lowercase.

4. Implement `first_word`:
   - Input: `&str`
   - Output: `&str` slice containing first word.
   - If there is no space, return the full string slice.

5. In `main`, finish the TODO section to avoid borrow conflicts:
   - Read data immutably.
   - Mutate data.
   - Read again.
   Keep borrow scopes clean so the compiler is happy.

6. Bonus:
   - Create a `longest_note` function using `&[String] -> Option<&String>`
     without allocating new strings.
*/

fn take_last_note(mut notes: Vec<String>) -> (String, Vec<String>) {
    // TODO: remove and return last note + remaining notes
    todo!("Implement ownership move + return")
}

fn count_keyword(notes: &[String], keyword: &str) -> usize {
    // TODO: immutable borrow only
    todo!("Implement immutable borrow counting")
}

fn normalize_notes(notes: &mut [String]) {
    // TODO: mutable borrow, normalize in place
    todo!("Implement mutable borrow normalization")
}

fn first_word(s: &str) -> &str {
    // TODO: return a slice of first word
    todo!("Implement string slice logic")
}

fn main() {
    let notes = vec![
        String::from("  Rust ownership rules  "),
        String::from("Borrowing prevents data races"),
        String::from("Slices are references into data"),
        String::from("  Move semantics transfer ownership  "),
    ];

    let (picked, mut notes) = take_last_note(notes);
    println!("Picked note: {picked}");

    let ownership_hits = count_keyword(&notes, "ownership");
    println!("Keyword hits before normalize: {ownership_hits}");

    // TODO: Step 5 - finish this flow without borrow checker issues.
    // 1) print first word of first note
    // 2) normalize all notes
    // 3) print first word again after normalization

    normalize_notes(&mut notes);
    let preview = first_word(&notes[0]);
    println!("First word after normalize: {preview}");
}
