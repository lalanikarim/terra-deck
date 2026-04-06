# Common Coding & Compilation Errors

Documentation of common pitfalls and their solutions encountered during development.

---

## Table of Contents

- [Rand Crate Issues](#rand-crate-issues)
- [Borrow Checker Issues](#borrow-checker-issues)
- [Tool Command Failures](#tool-command-failures)
- [Naming Conflicts](#naming-conflicts)
- [Test Method Issues](#test-method-issues)

---

## Rand Crate Issues

### Error 1: `unresolved import 'rand::prelude::thread_rng'`

**Cause**: `rand::prelude::thread_rng` doesn't exist in rand v0.10.0. The API changed.

**Symptom**:
```
error[E0432]: unresolved import `rand::prelude::thread_rng`
 --> game_core/src/ai.rs:4:5
  |
4 | use rand::prelude::thread_rng;
  |     ^^ no such `thread_rng` in `prelude`
```

**Solution**: Use `rand::rng()` or `rand::random_*()` functions instead.

**Before** (doesn't work):
```rust
use rand::prelude::thread_rng;

fn select_card() -> usize {
    let rng = thread_rng();
    rng.gen_range(0..10)
}
```

**After** (works with rand 0.10):
```rust
use rand::Rng;

fn select_card() -> usize {
    rand::random_range(0..10)
}
```

**Reference**: See [docs/RAND_KNOWLEDGE.md](RAND_KNOWLEDGE.md)

---

### Error 2: `no method named 'gen_range' found for struct 'ThreadRng'`

**Cause**: `gen_range` method doesn't exist on `ThreadRng` in rand v0.10.

**Symptom**:
```
error[E0599]: no method named `gen_range` found for struct `ThreadRng` in the current scope
```

**Solution**: Use `rand::random_range(range)` or `rng.random_range(range)` with `use rand::Rng`.

**Before** (doesn't work):
```rust
let rng = rand::rng();
let n = rng.gen_range(0..10);  // ❌ gen_range doesn't exist
```

**After** (works):
```rust
let n = rand::random_range(0..10);  // ✅ standalone function
```

Or with Rng trait:
```rust
use rand::Rng;

let rng = rand::rng();
let n = rng.random_range(0..10);  // ✅ random_range method
```

---

### Error 3: `cannot borrow 'rng' as mutable, as it is not declared as mutable`

**Cause**: Forgetting `mut` when using `rand::rng()`.

**Symptom**:
```
error[E0596]: cannot borrow `rng` as mutable, as it is not declared as mutable
  |
186 |     let rng = rand::rng();
    |         ^^^ not mutable
```

**Solution**: Add `mut` keyword:

```rust
let mut rng = rand::rng();  // ✅ must be mutable for Rng methods
```

---

## Borrow Checker Issues

### Error 4: `cannot borrow as mutable because it is also borrowed as immutable`

**Cause**: Rust borrow checker prevents simultaneous mutable and immutable borrows.

**Symptom**:
```
error[E0502]: cannot borrow `player_hand` as mutable because it is also borrowed as immutable
  --> game_core/src/systems.rs:50:9
   |
34 |         player_hand.get(player_card_idx),  // immutable borrow here
...
50 |         player_hand.cards[player_card_idx] = player_card_copy;  // ❌ mutable borrow
```

**Context**: In `resolve_combat_system`, we needed to:
1. Read cards from hand (immutable borrow via `get()`)
2. Modify cards with damage (mutable borrow)
3. Log card details (immutable borrow still active)

**Solution**: Clone the card data upfront for logging, then free the immutable borrow:

**Before** (doesn't compile):
```rust
let player_card = player_hand.get(0);  // holds immutable borrow
// ... calculate damage ...
player_hand.cards[0] = damaged_card;   // ❌ can't mutably borrow
```

**After** (compiles):
```rust
let player_card_info = player_hand.cards[0].clone();  // Clone, no borrow held
let opponent_card_info = opponent_hand.cards[0].clone();

// Do damage calculation with clones
let mut player_copy = player_card_info.clone();
let mut opponent_copy = opponent_card_info.clone();
apply_combat_damage(&player_copy, &mut opponent_copy);

// Update actual hands
player_hand.cards[0] = player_copy;  // ✅ mutable borrow works
opponent_hand.cards[0] = opponent_copy;
```

---

## Tool Command Failures

### Error 5: `sed: extra characters at the end of g command`

**Cause**: macOS uses BSD `sed` which has different syntax than GNU `sed`.

**Symptom**:
```
sed: 1: "game_core/src/lib.rs": extra characters at the end of g command
```

**Context**: Running sed commands with complex regex patterns, especially with newlines or special chars.

**Solution**: Use alternative tools:

**Option A**: Use `perl` instead (more consistent):
```bash
perl -i -pe 's/old_text/new_text/g' file.rs
```

**Option B**: Use `sed` with simpler patterns:
```bash
sed -i '' -e 's/old/new/g' file.rs  # Note the '' between -i and -e on macOS
```

**Option C**: Use Pi's native `write` or `edit` tools (most reliable):
```
(edit path="file.rs" edits=[{oldText: "...", newText: "..."}])
```

---

### Error 6: `sed` line deletion with special characters

**Cause**: Line numbers with context from previous commands can confuse shell parsing.

**Symptom**:
```
sed: 1: "game_core/src/systems.rs\n": extra characters at the end
```

**Solution**: Use absolute line numbers or perl:
```bash
perl -i -ne 'print unless $. == 177' file.rs  # Delete line 177
```

Or use the Pi `edit` tool with exact text matching.

---

## Naming Conflicts

### Error 7: `CombatResult` name collision

**Cause**: Both `combat.rs` and `combat_stats.rs` exported `CombatResult`.

**Symptom**:
```
error[E0432]: unresolved import `game_core::CombatResult`
|
= note: reference to module `combat_result` is ambiguous, could refer to:
          crate::combat::CombatResult
          crate::combat_stats::CombatResult
```

**Context**: `combat.rs` had CombatResult for damage multipliers, but we added GameResult in combat_stats.rs and accidentally aliased it.

**Before** (conflict):
```rust
// combat_stats.rs
pub enum GameResult { ... }

// lib.rs
pub use combat::CombatResult;
pub use combat_stats::CombatResult as GameResult;  // ❌ Wrong!
```

**After** (fixed):
```rust
// lib.rs
pub use combat::CombatResult;
pub use combat_stats::{CombatStats, GameResult, GameState};  // ✅ Separate name
```

---

## Test Method Issues

### Error 8: `take_damage` returns u8, not Card

**Cause**: Method modifies card in place and returns damage dealt, not modified card.

**Symptom**:
```
error[E0308]: mismatched types
   --> game_core/src/hand.rs:223:31
    |
223 |             hand.cards[idx] = card.take_damage(card.hp);
    |             ---------------   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Card`, found `u8`
```

**Context**: Testing `remove_dead_cards()` needed to damage cards to 0 HP.

**Before** (wrong):
```rust
let card = hand.cards[idx].clone();
hand.cards[idx] = card.take_damage(card.hp);  // ❌ take_damage returns u8, not Card
```

**After** (correct):
```rust
// take_damage() modifies card in place
let hp = hand.cards[i].hp;
hand.cards[i].take_damage(hp);  // ✅ modifies in place
```

---

### Error 9: `variable does not need to be mutable`

**Cause**: Declaring `mut` but not actually mutating the variable.

**Symptom**:
```
warning: variable does not need to be mutable
   --> game_core/src/combat/mod.rs:193:13
    |
193 |         let mut attacker = Card::new(...);
    |             ----^^^^^^^^
    |             |
    |             help: remove this `mut`
```

**Context**: In `test_apply_combat_damage`, attacker wasn't actually modified.

**Solution**: Remove `mut` if not modifying:
```rust
let attacker = Card::new(Suit::Hearts, Rank::Ten);  // ✅ no mut needed
let defender = Card::new(Suit::Clubs, Rank::Five);
```

---

## Quick Reference Summary

| Error Type | Key Pattern | Quick Fix |
|------------|------|-----|
| `rand::prelude::thread_rng` | Import error | Use `use rand::Rng;` + `rand::random_range()` |
| `gen_range` not found | Wrong method name | Use `random_range()` |
| `cannot borrow as mutable` | Borrow conflict | Clone data upfront, don't hold borrows |
| `sed: extra characters` | macOS BSD sed | Use `perl -i -pe` or Pi `edit` tool |
| `CombatResult` collision | Duplicate name | Use different names, careful with re-exports |
| `take_damage returns u8` | Method signature | Call method, don't assign return value |

---

## Prevention Tips

1. **Always read error messages fully**: Rust error messages are helpful and often include fixes
2. **Use Pi's edit tool over sed**: More reliable for complex replacements
3. **Clone before borrowing**: Makes borrow checker happy when you need both read and write
4. **Check rand API docs**: v0.10 has different API than older versions
5. **Be careful with module re-exports**: Watch for name conflicts between modules
6. **Understand return types**: `take_damage()` returns damage dealt, not the modified card

---

## References

- [docs/RAND_KNOWLEDGE.md](RAND_KNOWLEDGE.md) - Rand crate usage guide
- [docs/BEVY_KNOWLEDGE.md](BEVY_KNOWLEDGE.md) - Bevy ECS patterns
- [rust-lang.org Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [rand.rs documentation](https://docs.rs/rand) - Rand crate API reference
