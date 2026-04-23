# Design Document: Vesting Configurable Start Time

## Overview

This change adds an optional `start_time: Option<u64>` parameter to `ForgeVesting::initialize()`. When `None` is supplied the contract defaults to `env.ledger().timestamp()`, preserving existing behaviour exactly. When `Some(t)` is supplied the contract validates that `t >= env.ledger().timestamp()` and stores `t` as the vesting start. All elapsed-time calculations in `compute_vested` already use `config.start_time`, so no changes are needed there beyond the new validation at initialization.

## Architecture

The change is entirely contained within the `forge-vesting` crate. No new storage keys, no new contract interfaces, and no cross-contract interactions are introduced.

```mermaid
flowchart TD
    A[initialize called] --> B{start_time param}
    B -- None --> C[use env.ledger().timestamp()]
    B -- Some t --> D{t >= now?}
    D -- yes --> E[use t]
    D -- no --> F[return InvalidConfig]
    C --> G[store VestingConfig]
    E --> G
    G --> H[emit vesting_initialized event]
```

## Components and Interfaces

### `initialize()` signature change

```rust
pub fn initialize(
    env: Env,
    token: Address,
    beneficiary: Address,
    admin: Address,
    total_amount: i128,
    cliff_seconds: u64,
    duration_seconds: u64,
    start_time: Option<u64>,          // NEW
) -> Result<(), VestingError>
```

The resolution logic inserted before the `VestingConfig` construction:

```rust
let resolved_start = match start_time {
    None => env.ledger().timestamp(),
    Some(t) => {
        if t < env.ledger().timestamp() {
            return Err(VestingError::InvalidConfig);
        }
        t
    }
};
```

`VestingConfig { start_time: resolved_start, .. }` replaces the previous inline `env.ledger().timestamp()` call.

### `VestingConfig` — no structural change

`VestingConfig.start_time` already exists as a `u64` field. No migration or schema change is required.

### `compute_vested` — no change

The function already reads `config.start_time` for all elapsed-time arithmetic. It will automatically use the new resolved value without modification.

## Data Models

No new fields or storage keys. The only change is how `VestingConfig.start_time` is populated during `initialize()`.

| Field | Type | Change |
|---|---|---|
| `VestingConfig.start_time` | `u64` | Value now comes from caller-supplied `Option<u64>` or ledger timestamp |

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system — essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: None start_time defaults to ledger timestamp

*For any* ledger timestamp `now`, calling `initialize()` with `start_time = None` and then reading `get_config().start_time` should equal `now`.

**Validates: Requirements 1.2, 2.2**

### Property 2: Future start_time is stored verbatim

*For any* timestamp `t >= env.ledger().timestamp()`, calling `initialize()` with `start_time = Some(t)` and then reading `get_config().start_time` should equal `t`.

**Validates: Requirements 1.3, 2.1, 2.2**

### Property 3: Past start_time is rejected

*For any* timestamp `t < env.ledger().timestamp()`, calling `initialize()` with `start_time = Some(t)` should return `VestingError::InvalidConfig`.

**Validates: Requirements 1.4**

### Property 4: get_vesting_schedule reflects resolved start_time

*For any* valid `start_time` (None or Some(t >= now)), calling `get_vesting_schedule()` after initialization should return a `VestingSchedule` whose `start_time` equals the resolved value stored in `VestingConfig`.

**Validates: Requirements 2.3**

### Property 5: Vesting amount correctness relative to configured start_time

*For any* vesting config with a future `start_time` and *for any* ledger timestamp `now`:
- If `now < start_time + cliff_seconds`, `claim()` returns `CliffNotReached` (or `compute_vested` returns 0).
- If `now >= start_time + cliff_seconds` and `now < start_time + duration_seconds`, the vested amount equals `(total_amount * elapsed) / duration_seconds` where `elapsed = now - start_time`.
- If `now >= start_time + duration_seconds`, the vested amount equals `total_amount`.

**Validates: Requirements 3.1, 3.2, 3.3, 3.4**

## Error Handling

| Condition | Error |
|---|---|
| `start_time = Some(t)` where `t < env.ledger().timestamp()` | `VestingError::InvalidConfig` |

All existing `InvalidConfig` conditions (`total_amount <= 0`, `duration_seconds == 0`, `cliff_seconds > duration_seconds`) are unchanged and checked in the same guard.

## Testing Strategy

### Dual Testing Approach

Both unit tests and property-based tests are used. Unit tests cover specific examples and edge cases; property tests verify universal correctness across generated inputs.

### Property-Based Testing

The Rust property-based testing library **`proptest`** is used (added as a `[dev-dependency]`). Each property test runs a minimum of 100 iterations.

Each property test is annotated with a comment referencing its design property:
```
// Feature: vesting-configurable-start-time, Property N: <property text>
```

**Property test mapping:**

| Property | Test description |
|---|---|
| Property 1 | Generate random ledger timestamps; initialize with `None`; assert `config.start_time == now` |
| Property 2 | Generate random `t >= now`; initialize with `Some(t)`; assert `config.start_time == t` |
| Property 3 | Generate random `t < now`; initialize with `Some(t)`; assert `Err(InvalidConfig)` |
| Property 4 | Generate valid configs; assert `get_vesting_schedule().start_time == get_config().start_time` |
| Property 5 | Generate valid configs with future start; generate `now` across full timeline; assert vested amount matches formula |

### Unit Tests

Unit tests cover:
- `initialize()` with `start_time = None` (present-time default, backward-compat)
- `initialize()` with `start_time = Some(future)` (accepted)
- `initialize()` with `start_time = Some(past)` (rejected with `InvalidConfig`)
- `initialize()` with `start_time = Some(now)` (boundary — accepted)
- `claim()` before cliff when start is in the future (returns `CliffNotReached`)
- `claim()` at and after cliff with future start (returns correct amount)
- `get_vesting_schedule()` reflects the configured start time
