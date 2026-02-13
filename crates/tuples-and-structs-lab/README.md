# tuples-and-structs-lab

Practice lab for:
- tuple structs and regular structs
- associated functions without `self` and with `self`/`&mut self`
- ownership and borrowing
- data flow through moves/mutable updates
- pattern matching

## Tasks

1. Medium: `GridPoint::origin` (associated function without `self`)
2. Medium: `GridPoint::shift` (associated function with `self`)
3. Medium: `WorkItem::new` (constructor, ownership of `String`)
4. Medium: `WorkItem::retitle_and_swap_tags` (mutable borrowing + moves)
5. Medium: `classify_effort` (pattern matching rules)
6. Bonus Advanced: `find_match_with_trace` (borrowing + lifetimes + tuple query + matching)

All exercise stubs are in `src/lib.rs` and use `todo!()`.
Run tests with:

```bash
cargo test -p tuples-and-structs-lab
```
