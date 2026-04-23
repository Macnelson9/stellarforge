# Requirements Document

## Introduction

The `forge-multisig` contract exposes `approve()`, `reject()`, and `execute()` entry points that each read a proposal from persistent storage and return `Err(MultisigError::ProposalNotFound)` when the proposal does not exist. Currently there are no tests that exercise this error path directly. A regression that alters the storage key format (e.g. a change to `DataKey::Proposal`) could silently miss the stored proposal and produce a panic or an unexpected error instead of the explicit `ProposalNotFound` variant. This feature adds targeted tests that confirm each of the three functions returns the correct error — and does not panic — when called with a non-existent proposal ID.

## Glossary

- **MultisigContract**: The Soroban smart contract under test, located at `contracts/forge-multisig/src/lib.rs`.
- **MultisigError**: The contract's error enum; the relevant variant is `MultisigError::ProposalNotFound` (discriminant 4).
- **proposal_id**: A `u64` identifier used to look up a `Proposal` in persistent storage via `DataKey::Proposal(proposal_id)`.
- **Non-existent proposal**: A `proposal_id` for which no entry has ever been written to persistent storage in the current contract instance (e.g. `9999` on a freshly initialized multisig).
- **Owner**: An address registered during `initialize()` that is authorised to call `approve()`, `reject()`, and `execute()`.
- **try_approve / try_reject / try_execute**: The Soroban test-client methods that return `Result` instead of panicking, used to assert error variants in tests.

## Requirements

### Requirement 1: approve() returns ProposalNotFound for a non-existent proposal

**User Story:** As a contract integrator, I want `approve()` to return `Err(MultisigError::ProposalNotFound)` when the proposal ID does not exist, so that callers can distinguish a missing proposal from a panic or an unrelated error.

#### Acceptance Criteria

1. WHEN `approve()` is called with a `proposal_id` that has never been stored, THE MultisigContract SHALL return `Err(MultisigError::ProposalNotFound)`.
2. WHEN `approve()` is called with a non-existent `proposal_id`, THE MultisigContract SHALL complete without panicking.

### Requirement 2: reject() returns ProposalNotFound for a non-existent proposal

**User Story:** As a contract integrator, I want `reject()` to return `Err(MultisigError::ProposalNotFound)` when the proposal ID does not exist, so that callers can distinguish a missing proposal from a panic or an unrelated error.

#### Acceptance Criteria

1. WHEN `reject()` is called with a `proposal_id` that has never been stored, THE MultisigContract SHALL return `Err(MultisigError::ProposalNotFound)`.
2. WHEN `reject()` is called with a non-existent `proposal_id`, THE MultisigContract SHALL complete without panicking.

### Requirement 3: execute() returns ProposalNotFound for a non-existent proposal

**User Story:** As a contract integrator, I want `execute()` to return `Err(MultisigError::ProposalNotFound)` when the proposal ID does not exist, so that callers can distinguish a missing proposal from a panic or an unrelated error.

#### Acceptance Criteria

1. WHEN `execute()` is called with a `proposal_id` that has never been stored, THE MultisigContract SHALL return `Err(MultisigError::ProposalNotFound)`.
2. WHEN `execute()` is called with a non-existent `proposal_id`, THE MultisigContract SHALL complete without panicking.
