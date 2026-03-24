# StellarForge Composability Guide

StellarForge contracts are designed as independent primitives that work well together. This guide walks through a real-world scenario combining multiple contracts to build a fully governed DAO.

---

## Scenario: A DAO Treasury with Governed Payments and Contributor Streams

This scenario uses three contracts together:

- **`forge-governor`** — token-weighted voting to approve decisions
- **`forge-multisig`** — multi-owner treasury that executes approved transfers
- **`forge-stream`** — real-time per-second salary payments to contributors

The flow looks like this:

```
Token Holders
     │
     ▼
forge-governor  ──(proposal passes)──▶  forge-multisig  ──(execute)──▶  forge-stream
  (vote)                                  (treasury)                    (contributor salary)
```

---

## Step 1: Deploy and Initialize Each Contract

### Governor

```rust
let config = GovernorConfig {
    vote_token: dao_token_address,  // governance token
    voting_period: 604_800,         // 7 days in seconds
    quorum: 1_000_000,              // 1M tokens minimum participation
    timelock_delay: 172_800,        // 2-day execution delay
};
governor_client.initialize(&config);
```

### Multisig Treasury

```rust
// 3-of-5 multisig with a 24-hour timelock
multisig_client.initialize(
    &vec![&env, council_a, council_b, council_c, council_d, council_e],
    &3,       // threshold: 3 approvals required
    &86_400,  // 24-hour timelock delay
);
```

### Stream (no initialization needed — stateless per stream)

`forge-stream` is ready to use after deployment. Each `create_stream` call is self-contained.

---

## Step 2: Propose a Contributor Salary via Governance

A DAO member proposes paying a contributor via a token stream. The proposal description encodes the intended action off-chain (the governor itself doesn't execute cross-contract calls — it signals intent).

```rust
let proposal_id = governor_client.propose(
    &proposer,
    &String::from_str(&env, "Fund contributor salary stream"),
    &String::from_str(&env, "Approve 1000 USDC/day stream to Alice for 90 days. \
                              Multisig to fund forge-stream contract and create stream."),
)?;
```

Token holders vote during the `voting_period`:

```rust
// Each voter supplies their token balance as weight
governor_client.vote(&voter_a, &proposal_id, &true, &500_000)?;
governor_client.vote(&voter_b, &proposal_id, &true, &700_000)?;
```

After the voting period ends, anyone can finalize:

```rust
let state = governor_client.finalize(&proposal_id)?;
// state == ProposalState::Passed
```

After the `timelock_delay`, mark it executed on-chain:

```rust
governor_client.execute(&executor, &proposal_id)?;
```

---

## Step 3: Multisig Council Funds the Stream

With governance approval recorded, the multisig council now acts. One council member proposes transferring funds to the stream contract:

```rust
// Transfer enough to fund 90 days at ~11.574 USDC/sec (1000 USDC/day)
// 90 days = 7_776_000 seconds, rate = 11_574 stroops/sec ≈ 1000 USDC/day
let stream_funding_amount: i128 = 11_574 * 7_776_000;

let ms_proposal_id = multisig_client.propose(
    &council_a,
    &stream_contract_address,  // destination: the stream contract
    &usdc_token_address,
    &stream_funding_amount,
)?;
```

Other council members approve:

```rust
multisig_client.approve(&council_b, &ms_proposal_id)?;
multisig_client.approve(&council_c, &ms_proposal_id)?;
// Threshold of 3 reached — timelock starts
```

After the 24-hour multisig timelock:

```rust
multisig_client.execute(&council_a, &ms_proposal_id)?;
// USDC is now in the stream contract's balance
```

---

## Step 4: Create the Contributor Stream

With funds in the stream contract, a council member (acting as sender) creates the stream for the contributor:

```rust
let stream_id = stream_client.create_stream(
    &council_a,           // sender (must authorize token transfer)
    &usdc_token_address,
    &alice_address,       // recipient: the contributor
    &11_574,              // rate_per_second (~1000 USDC/day)
    &7_776_000,           // duration: 90 days in seconds
)?;
```

Alice can now withdraw her accrued salary at any time:

```rust
// Alice calls this whenever she wants to claim accrued tokens
let withdrawn = stream_client.withdraw(&stream_id)?;
```

The sender can pause the stream if needed (e.g., contributor goes on leave):

```rust
stream_client.pause_stream(&stream_id)?;
// ... later ...
stream_client.resume_stream(&stream_id)?;
```

Or cancel it entirely if the contributor leaves, automatically refunding unstreamed tokens:

```rust
stream_client.cancel_stream(&stream_id)?;
// Alice receives accrued tokens; council_a is refunded the rest
```

---

## Composability Patterns

### Pattern 1: Governor → Multisig → Stream (shown above)
Governance votes on budget decisions. The multisig council executes them. Streams handle ongoing payments.

### Pattern 2: Governor → Multisig (treasury management)
Use `forge-governor` to vote on any treasury transfer, then `forge-multisig` to execute it with multi-owner safety. No stream needed for one-time payments.

### Pattern 3: Multisig → Vesting (team token grants)
The multisig treasury funds a `forge-vesting` contract for a new team member. The multisig proposes and executes the token transfer to the vesting contract, which then handles the cliff and linear unlock.

```rust
// Multisig proposes funding a vesting contract
let pid = multisig_client.propose(
    &council_a,
    &vesting_contract_address,
    &dao_token_address,
    &1_000_000_0000000, // 1M tokens
)?;
```

### Pattern 4: Oracle → Governor (price-gated proposals)
Off-chain tooling can read `forge-oracle` price data to determine voting weight or proposal validity before submitting to `forge-governor`. For example, a UI could gate proposal creation to wallets holding a minimum USD value of governance tokens.

---

## Key Integration Notes

- **forge-governor does not execute cross-contract calls directly.** It records proposal state on-chain. Your off-chain tooling or a separate coordinator contract reads the `Executed` state and triggers downstream actions.
- **forge-multisig holds tokens in its own contract balance.** Fund it by transferring tokens to its contract address before calling `propose`.
- **forge-stream pulls tokens from the sender on `create_stream`.** The sender must have sufficient balance and authorize the transfer.
- **Timelocks are additive.** In the full DAO scenario above, there are two timelocks: the governor's `timelock_delay` and the multisig's `timelock_delay`. Plan your governance timelines accordingly.

---

## Further Reading

- [State Diagrams](state-diagrams.md) — lifecycle diagrams for `forge-vesting`, `forge-stream`, and `forge-governor`
- [README](../README.md) — contract API reference and event documentation
