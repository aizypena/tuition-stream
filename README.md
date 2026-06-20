# EduWallet

Programmable scholarship disbursement and transparent fund distribution built on Stellar.

---

## Overview

EduWallet enables scholarship foundations to distribute education stipends to students through a transparent, auditable workflow powered by Stellar and Soroban smart contracts.

Instead of relying on spreadsheets, email approvals, bank transfers, and manual receipt reviews, scholarship funds are managed through an on-chain approval process where every action is recorded and verifiable.

---

## Problem

A scholarship recipient at a public university in Philippines receives a monthly education stipend, but the scholarship foundation spends several days every month manually reviewing receipts and bank transfers to verify that funds are used for tuition, books, and school supplies.

This process creates administrative overhead, slows down fund distribution, and makes auditing difficult.

---

## Solution

EduWallet uses Soroban smart contracts on Stellar to manage scholarship disbursement workflows.

A scholarship foundation creates a scholarship allocation for a student. The student submits a withdrawal request, the foundation approves it, and the student claims the stipend. Every step is recorded on-chain, creating a transparent and auditable funding trail.

---

## Why Stellar?

Stellar provides:

* Fast transaction settlement
* Low transaction fees
* Transparent on-chain records
* Soroban smart contract programmability
* Future support for USDC-based scholarship payments
* Compliance and clawback capabilities when required

---

## Stellar Features Used

### Soroban Smart Contracts

Used to manage scholarship state and approval workflows.

### USDC Transfers (Future Enhancement)

Can be used for real scholarship payouts.

### Trustlines (Future Enhancement)

Allows students to receive regulated educational assets.

### Clawback / Compliance (Future Enhancement)

Supports compliance requirements for scholarship providers.

---

## Target Users

### Students

* Ages 18–25
* Scholarship recipients
* Public universities in Manila, Philippines
* Require timely access to educational stipends

### Scholarship Foundations

* Educational NGOs
* Corporate CSR programs
* University scholarship offices

Benefits:

* Reduced administrative workload
* Transparent auditing
* Faster scholarship disbursement

---

## MVP Workflow

### Step 1 — Foundation Creates Scholarship

The foundation creates a scholarship allocation for a student.

### Step 2 — Student Requests Withdrawal

The student submits a withdrawal request.

### Step 3 — Foundation Approves Request

The scholarship administrator reviews and approves the request.

### Step 4 — Student Claims Funds

The student claims the approved scholarship.

### Step 5 — On-Chain Audit Trail

The scholarship status is permanently stored on Stellar.

---

## Smart Contract Functions

### initialize

Registers the scholarship foundation administrator.

```rust
initialize(admin)
```

### create_scholarship

Creates a scholarship allocation.

```rust
create_scholarship(student, amount)
```

### request_withdrawal

Allows a student to request stipend release.

```rust
request_withdrawal(student)
```

### approve_request

Allows the foundation to approve the withdrawal request.

```rust
approve_request(student)
```

### claim

Allows the student to claim an approved scholarship.

```rust
claim(student)
```

### get_status

Returns the scholarship state.

```rust
get_status(student)
```

---

## Project Architecture

```text
Foundation
    │
    ▼
Create Scholarship
    │
    ▼
Student Request
    │
    ▼
Foundation Approval
    │
    ▼
Student Claim
    │
    ▼
On-Chain Record
```

---

## Timeline

### Week 1

* Project setup
* Smart contract development
* Unit testing

### Week 2

* Frontend development
* Wallet integration
* Testnet deployment

### Week 3

* UI improvements
* Demo preparation
* Documentation

---

## Vision

EduWallet aims to become the financial infrastructure layer for scholarship programs across Southeast Asia.

Future versions will support:

* USDC scholarship payments
* University merchant payments
* Tuition-only spending controls
* AI receipt verification
* Education voucher assets
* Cross-border scholarship programs

---

## Prerequisites

### Rust

Install Rust:

```bash
rustup update
```

### Soroban CLI

Install Soroban CLI:

```bash
cargo install --locked soroban-cli
```

Verify installation:

```bash
soroban --version
```

---

## Build Contract

Compile the smart contract:

```bash
soroban contract build
```

Output:

```text
target/wasm32-unknown-unknown/release/eduwallet.wasm
```

---

## Run Tests

Execute all unit tests:

```bash
cargo test
```

Expected result:

```text
5 passed
0 failed
```

---

## Deploy to Stellar Testnet

Configure identity:

```bash
soroban config identity generate eduwallet-admin
```

Deploy contract:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/eduwallet.wasm \
  --source eduwallet-admin \
  --network testnet
```

The command returns a Contract ID.

Example:

```text
CBK7YJ2P4F5EXAMPLECONTRACTID
```

---

## Sample Contract Calls

### Initialize Foundation

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source eduwallet-admin \
  --network testnet \
  -- initialize \
  --admin GADMINADDRESS
```

### Create Scholarship

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source eduwallet-admin \
  --network testnet \
  -- create_scholarship \
  --student GSTUDENTADDRESS \
  --amount 100
```

### Request Withdrawal

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source student-wallet \
  --network testnet \
  -- request_withdrawal \
  --student GSTUDENTADDRESS
```

### Approve Request

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source eduwallet-admin \
  --network testnet \
  -- approve_request \
  --student GSTUDENTADDRESS
```

### Claim Scholarship

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source student-wallet \
  --network testnet \
  -- claim \
  --student GSTUDENTADDRESS
```

### View Scholarship Status

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source eduwallet-admin \
  --network testnet \
  -- get_status \
  --student GSTUDENTADDRESS
```

---

## Future Enhancements

### Merchant-Restricted Spending

Allow scholarship funds to be spent only at:

* Universities
* Bookstores
* Educational suppliers

### AI Receipt Verification

Students upload receipts and AI verifies:

* Merchant name
* Amount
* Education-related purchases

### Compliance Controls

Use Stellar asset controls to support:

* Clawback
* Spending restrictions
* Scholarship revocation

### Mobile Wallet

A mobile-first experience for students with:

* QR payments
* Scholarship balance tracking
* Push notifications

---

## License

MIT License

Copyright (c) 2026 EduWallet

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files to deal in the Software without restriction.

Link: https://stellar.expert/explorer/testnet/contract/CD23URHYEFXZLRZUPNYMTNULVL3O3YZBZHLBWEJOQKRYQ4BZBT7ML6VB
