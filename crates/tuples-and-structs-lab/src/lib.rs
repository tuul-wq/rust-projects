#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPoint(pub i32, pub i32);

impl GridPoint {
    // Task 1 (Medium):
    // Implement an associated function without `self`.
    // Return the coordinate origin `(0, 0)`.
    pub fn origin() -> Self {
      Self(0, 0)
    }

    // Task 2 (Medium):
    // Implement an associated function with `self` that consumes the point.
    // Apply the `(dx, dy)` delta and return a new point.
    pub fn shift(self, delta: (i32, i32)) -> Self {
        Self(self.0 + delta.0, self.1 + delta.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItem {
    title: String,
    owner: String,
    tags: (String, String),
    estimate_hours: u8,
    done: bool,
}

impl WorkItem {
    // Task 3 (Medium):
    // Implement an associated function without `self` that constructs a new `WorkItem`.
    // - `done` must start as `false`.
    pub fn new(title: String, owner: String, tags: (String, String), estimate_hours: u8) -> Self {
        Self {
          title,
          owner,
          tags,
          estimate_hours,
          done: false,
        }
    }

    // Task 4 (Medium):
    // Implement a method with `&mut self`:
    // - Move the old title out and return it.
    // - Replace the title with `new_title`.
    // - Swap the two tags (primary <-> secondary) to model data flow updates.
    pub fn retitle_and_swap_tags(&mut self, new_title: String) -> String {
      let old_title = std::mem::replace(&mut self.title, new_title);
      std::mem::swap(&mut self.tags.0, &mut self.tags.1);

      old_title
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn tags(&self) -> (&str, &str) {
        (&self.tags.0, &self.tags.1)
    }

    pub fn estimate_hours(&self) -> u8 {
        self.estimate_hours
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effort {
    Small,
    Medium,
    Large,
}

// Task 5 (Medium):
// Implement with pattern matching.
// Rules:
// - Any completed item is `Small`.
// - For open items: 0..=2 => `Small`, 3..=5 => `Medium`, otherwise => `Large`.
pub fn classify_effort(item: &WorkItem) -> Effort {
    match item {
        WorkItem { done: true, .. } => Effort::Small,
        WorkItem { estimate_hours: 0..=2, .. } => Effort::Small,
        WorkItem { estimate_hours: 3..=5, .. } => Effort::Medium,
        _ => Effort::Large,
    }
}

// Similar Task 6 (without lifetimes):
// Return the first matching item's index and a tuple-struct "trace point".
// Input query:
// - query.0 = title prefix
// - query.1 = open_only flag
//
// Match conditions:
// - title must start with the prefix
// - if open_only is true, only match items where done == false
//
// Return:
// - index of matching item (usize)
// - GridPoint(index_in_slice, estimate_hours_as_i32)
pub fn find_match_with_trace(
    items: &[WorkItem],
    query: (&str, bool),
) -> Option<(usize, GridPoint)> {
    let (prefix, open_only) = query;

    for (index, item) in items.iter().enumerate() {
        if item.title().starts_with(prefix) && (!open_only || !item.is_done()) {
            return Some((index, GridPoint(index as i32, item.estimate_hours() as i32)));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_origin_returns_zero_point() {
        assert_eq!(GridPoint::origin(), GridPoint(0, 0));
    }

    #[test]
    fn task_2_shift_applies_delta() {
        let p = GridPoint(2, -3);
        assert_eq!(p.shift((4, 10)), GridPoint(6, 7));
    }

    #[test]
    fn task_3_new_initializes_fields() {
        let item = WorkItem::new(
            String::from("Refactor parser"),
            String::from("yara"),
            (String::from("backend"), String::from("rust")),
            4,
        );

        assert_eq!(item.title(), "Refactor parser");
        assert_eq!(item.owner(), "yara");
        assert_eq!(item.tags(), ("backend", "rust"));
        assert_eq!(item.estimate_hours(), 4);
        assert!(!item.is_done());
    }

    #[test]
    fn task_4_retitle_swaps_tags_and_returns_old_title() {
        let mut item = WorkItem::new(
            String::from("old title"),
            String::from("owner"),
            (String::from("alpha"), String::from("beta")),
            2,
        );

        let old_title = item.retitle_and_swap_tags(String::from("new title"));

        assert_eq!(old_title, "old title");
        assert_eq!(item.title(), "new title");
        assert_eq!(item.tags(), ("beta", "alpha"));
    }

    #[test]
    fn task_5_classify_effort_uses_pattern_matching_rules() {
        let quick = WorkItem::new(
            String::from("Write changelog"),
            String::from("ana"),
            (String::from("docs"), String::from("release")),
            1,
        );
        assert_eq!(classify_effort(&quick), Effort::Small);

        let medium = WorkItem::new(
            String::from("Add parser tests"),
            String::from("ana"),
            (String::from("backend"), String::from("tests")),
            5,
        );
        assert_eq!(classify_effort(&medium), Effort::Medium);

        let large = WorkItem::new(
            String::from("Rewrite query planner"),
            String::from("ana"),
            (String::from("backend"), String::from("perf")),
            8,
        );
        assert_eq!(classify_effort(&large), Effort::Large);
    }

    #[test]
    fn task_6_bonus_find_match_with_trace_returns_index_and_trace() {
        let items = vec![
            WorkItem::new(
                String::from("Ship API docs"),
                String::from("niko"),
                (String::from("docs"), String::from("public")),
                2,
            ),
            WorkItem::new(
                String::from("Stabilize parser"),
                String::from("niko"),
                (String::from("backend"), String::from("rust")),
                6,
            ),
        ];

        let (index, trace) = find_match_with_trace(&items, ("Sta", true))
            .expect("expected a matching open item");

        assert_eq!(index, 1);
        assert_eq!(trace, GridPoint(1, 6));
    }
}
