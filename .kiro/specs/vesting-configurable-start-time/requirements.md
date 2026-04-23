# Requirements Document

## Introduction

The `forge-vesting` contract currently sets `start_time` to `env.ledger().timestamp()` at initialization, meaning vesting always begins immediately at deployment. Real-world use cases — such as employee vesting that should begin on a future hire date — require the ability to pre-schedule a future start time. This feature adds an optional `start_time` parameter to `initialize()` so deployers can specify when vesting should begin, while preserving the existing default behaviour when no start time is provided.

## Glossary

- **ForgeVesting**: The Soroban smart contract that manages token vesting schedules.
- **VestingConfig**: The on-chain storage struct that holds all configuration for an active vesting schedule.
- **initialize()**: The one-time setup function that configures a vesting schedule.
- **start_time**: The ledger timestamp (seconds since Unix epoch) at which vesting begins and elapsed time starts accumulating.
- **cliff_seconds**: The number of seconds after `start_time` before any tokens unlock.
- **duration_seconds**: The total length of the vesting schedule in seconds, measured from `start_time`.
- **compute_vested**: The internal function that calculates how many tokens have vested based on elapsed time since `start_time`.
- **InvalidConfig**: The `VestingError` variant returned when initialization parameters are logically invalid.
- **Ledger timestamp**: The current on-chain time returned by `env.ledger().timestamp()`, used as the reference for "now".

## Requirements

### Requirement 1: Optional Start Time Parameter

**User Story:** As a contract deployer, I want to optionally specify a future `start_time` when initializing a vesting schedule, so that I can pre-schedule vesting to begin at a known future date without needing to deploy at exactly that moment.

#### Acceptance Criteria

1. THE `ForgeVesting` SHALL accept an optional `start_time: Option<u64>` parameter in `initialize()`.
2. WHEN `start_time` is `None`, THE `ForgeVesting` SHALL set `VestingConfig.start_time` to `env.ledger().timestamp()`, preserving current behaviour.
3. WHEN `start_time` is `Some(t)` and `t >= env.ledger().timestamp()`, THE `ForgeVesting` SHALL set `VestingConfig.start_time` to `t`.
4. WHEN `start_time` is `Some(t)` and `t < env.ledger().timestamp()`, THE `ForgeVesting` SHALL return `VestingError::InvalidConfig`.
5. WHEN `start_time` is `Some(t)` and `t == env.ledger().timestamp()`, THE `ForgeVesting` SHALL set `VestingConfig.start_time` to `t` and treat it as a valid present-time start.

### Requirement 2: VestingConfig Storage

**User Story:** As a contract integrator, I want `VestingConfig` to store the actual resolved start time, so that downstream reads via `get_config()` and `get_vesting_schedule()` always reflect the true start time of the schedule.

#### Acceptance Criteria

1. THE `VestingConfig` SHALL store the resolved `start_time` value (whether defaulted or explicitly provided) after a successful `initialize()` call.
2. WHEN `get_config()` is called after initialization, THE `ForgeVesting` SHALL return a `VestingConfig` whose `start_time` field equals the value passed to `initialize()` (or `env.ledger().timestamp()` if `None` was passed).
3. WHEN `get_vesting_schedule()` is called after initialization, THE `ForgeVesting` SHALL return a `VestingSchedule` whose `start_time` field equals the resolved start time.

### Requirement 3: Claim Correctness with Configured Start Time

**User Story:** As a beneficiary, I want `claim()` to correctly compute vested tokens relative to the configured `start_time`, so that I cannot claim tokens before vesting has actually begun and tokens unlock on the correct schedule.

#### Acceptance Criteria

1. WHEN the current ledger timestamp is before `VestingConfig.start_time + cliff_seconds`, THE `ForgeVesting` SHALL return `VestingError::CliffNotReached` from `claim()`.
2. WHEN the current ledger timestamp is at or after `VestingConfig.start_time + cliff_seconds`, THE `ForgeVesting` SHALL allow `claim()` to proceed and return the correct vested amount.
3. WHEN `compute_vested` is called with a `now` value less than `VestingConfig.start_time`, THE `ForgeVesting` SHALL return `0` vested tokens.
4. WHEN `compute_vested` is called with a `now` value equal to `VestingConfig.start_time + duration_seconds`, THE `ForgeVesting` SHALL return `total_amount` as the vested amount.

### Requirement 4: Documentation Update

**User Story:** As a developer integrating the contract, I want the `initialize()` doc comment to describe the new `start_time` parameter, so that I understand its semantics, default behaviour, and validation rules without reading the source code.

#### Acceptance Criteria

1. THE `ForgeVesting` `initialize()` function doc comment SHALL document the `start_time` parameter, including its type (`Option<u64>`), default behaviour when `None`, and the `InvalidConfig` error condition when a past timestamp is supplied.
