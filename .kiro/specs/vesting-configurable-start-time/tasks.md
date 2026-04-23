# Implementation Plan: Vesting Configurable Start Time

## Overview

Incremental implementation of the optional `start_time` parameter for `ForgeVesting::initialize()`. Each task builds on the previous and ends with all components wired together and tested.

## Tasks

- [ ] 1. Add `start_time: Option<u64>` parameter to `initialize()` and resolve it
  - In `contracts/forge-vesting/src/lib.rs`, add `start_time: Option<u64>` as the last parameter of `initialize()`
  - Insert resolution logic: `None` → `env.ledger().timestamp()`; `Some(t)` where `t < env.ledger().timestamp()` → `return Err(VestingError::InvalidConfig)`; `Some(t)` → `t`
  - Replace the inline `env.ledger().timestamp()` in the `VestingConfig` construction with `resolved_start`
  - The existing `InvalidConfig` guard (`total_amount <= 0`, `duration_seconds == 0`, `cliff_seconds > duration_seconds`) remains unchanged
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1_

  - [ ]* 1.1 Write unit tests for start_time resolution
    - Test `None` → defaults to ledger timestamp (backward-compat)
    - Test `Some(future_t)` → stored verbatim
    - Test `Some(past_t)` → returns `VestingError::InvalidConfig`
    - Test `Some(now)` → accepted (boundary edge case)
    - _Requirements: 1.2, 1.3, 1.4, 1.5_

- [ ] 2. Update `initialize()` doc comment
  - Add `start_time` to the `# Parameters` section: document `Option<u64>` type, `None` default behaviour, and the `InvalidConfig` error when a past timestamp is supplied
  - _Requirements: 4.1_

- [ ] 3. Add `proptest` dev-dependency and write property-based tests
  - Add `proptest = "1"` under `[dev-dependencies]` in `contracts/forge-vesting/Cargo.toml`

  - [ ]* 3.1 Property 1 — None defaults to ledger timestamp
    - Generate random ledger timestamps; call `initialize()` with `start_time = None`; assert `get_config().start_time == now`
    - Annotate: `// Feature: vesting-configurable-start-time, Property 1: None start_time defaults to ledger timestamp`
    - _Requirements: 1.2, 2.2_

  - [ ]* 3.2 Property 2 — Future start_time stored verbatim
    - Generate random `t >= now`; call `initialize()` with `start_time = Some(t)`; assert `get_config().start_time == t`
    - Annotate: `// Feature: vesting-configurable-start-time, Property 2: Future start_time is stored verbatim`
    - _Requirements: 1.3, 2.1, 2.2_

  - [ ]* 3.3 Property 3 — Past start_time rejected
    - Generate random `t < now`; call `initialize()` with `start_time = Some(t)`; assert `Err(VestingError::InvalidConfig)`
    - Annotate: `// Feature: vesting-configurable-start-time, Property 3: Past start_time is rejected`
    - _Requirements: 1.4_

  - [ ]* 3.4 Property 4 — get_vesting_schedule reflects resolved start_time
    - Generate valid configs (None and Some(future)); assert `get_vesting_schedule().start_time == get_config().start_time`
    - Annotate: `// Feature: vesting-configurable-start-time, Property 4: get_vesting_schedule reflects resolved start_time`
    - _Requirements: 2.3_

  - [ ]* 3.5 Property 5 — Vesting amount correctness relative to configured start_time
    - Generate valid configs with future `start_time`; generate `now` across the full timeline (before cliff, between cliff and duration, at duration); assert vested amount matches the linear formula
    - Covers edge cases: `now < start_time` → 0 vested; `now == start_time + duration_seconds` → `total_amount`
    - Annotate: `// Feature: vesting-configurable-start-time, Property 5: Vesting amount correctness relative to configured start_time`
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [ ] 4. Write unit tests for claim() with a future start_time
  - Test `claim()` before cliff when `start_time` is in the future → `VestingError::CliffNotReached`
  - Test `claim()` at exactly `start_time + cliff_seconds` → succeeds and returns correct amount
  - Test `claim()` at `start_time + duration_seconds` → returns `total_amount`
  - Test `get_vesting_schedule()` returns the configured `start_time`
  - _Requirements: 3.1, 3.2, 2.3_

- [ ] 5. Checkpoint — Ensure all tests pass
  - Run `cargo test -p forge-vesting` and confirm all existing and new tests pass. Ask the user if any questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for a faster MVP
- `compute_vested` requires no changes — it already reads `config.start_time`
- `VestingConfig` requires no schema changes — `start_time: u64` already exists
- All existing callers of `initialize()` must pass `None` as the new last argument to preserve current behaviour
