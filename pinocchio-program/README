# Instructions for running the tests in this repo

### 1. Create the program Id

```bash
solana-keygen new -s -o ./target/deploy/pinocchio_program-keypair.json
```

### 2. Log out the program Id

```bash
solana address -k ./target/deploy/pinocchio_program-keypair.json
```

you should see something like this `9XZcaw8CScYtsgfs7sw76qCqddycy7y41BiATa62KygN` in your terminal

### 3. Place your new Program Id in the declare_id macro in `src/lib.rs`

```rust
pinocchio_pubkey::declare_id!("9XZcaw8CScYtsgfs7sw76qCqddycy7y41BiATa62KygN");
```

### 4. You're ready to build and test the program - the unit tests are in `src/test/mod.rs`

```bash
cargo build-sbf && cargo test -- --no-capture
```

### You should see something like this in the terminal

```text
Init transaction sucessful
CUs Consumed: 3079
test test::tests::create_global_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
```
