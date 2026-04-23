# Implementation Plan: multisig-proposal-not-found-tests

## Overview

Add three unit tests to the existing `#[cfg(test)] mod tests` block in `contracts/forge-multisig/src/lib.rs`. Each test calls `try_approve`, `try_reject`, or `try_execute` with a non-existent proposal ID and asserts `Err(Ok(MultisigError::ProposalNotFound))`.

## Tasks

- [ ] 1. Add test for approve() with a non-existent proposal ID
  - Inside `mod tests` in `contracts/forge-multisig/src/lib.rs`, add `test_approve_non_existent_proposal_returns_not_found`
  - Use `setup_2of3`, call `client.try_approve(&o1, &9999_u64)`, assert `Err(Ok(MultisigError::ProposalNotFound))`
  - _Requirements: 1.1, 1.2_

- [ ] 2. Add test for reject() with a non-existent proposal ID
  - Inside `mod tests`, add `test_reject_non_existent_proposal_returns_not_found`
  - Use `setup_2of3`, call `client.try_reject(&o1, &9999_u64)`, assert `Err(Ok(MultisigError::ProposalNotFound))`
  - _Requirements: 2.1, 2.2_

- [ ] 3. Add test for execute() with a non-existent proposal ID
  - Inside `mod tests`, add `test_execute_non_existent_proposal_returns_not_found`
  - Use `setup_2of3`, call `client.try_execute(&o1, &9999_u64)`, assert `Err(Ok(MultisigError::ProposalNotFound))`
  - _Requirements: 3.1, 3.2_

- [ ] 4. Checkpoint — ensure all tests pass
  - Run `cargo test -p forge-multisig` and confirm all three new tests pass alongside the existing suite. Ask the user if any questions arise.
