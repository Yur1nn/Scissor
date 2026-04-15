# Optimization Plan for Scissor

## 1. Avoid Unnecessary clone()
- Refactor block/world/permission APIs to use references or smart pointers where possible.

## 2. Use &str Instead of String
- Change function arguments and HashMap keys from String to &str where feasible.

## 3. Use Faster HashMap for Block Storage
- Replace std::collections::HashMap with ahash::AHashMap for block and world storage.

## 4. Compact Player Name Storage
- Consider interning player names or using Cow<str> for reduced allocations.

## 5. Dense World Storage
- If world is dense, use Vec or array for block storage instead of HashMap.

## 6. Inline Small Methods
- Add #[inline] to small, hot methods (e.g., permission checks).

## 7. Concurrency (Future)
- If needed, use dashmap for concurrent world/permission access.

## 8. Profile and Benchmark
- Add notes for future profiling and benchmarking.

---

This plan will be implemented step by step, with code and doc updates as needed.
