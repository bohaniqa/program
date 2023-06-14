# Environment Stup

Set Solana cluster and wallet path (boq keypair).
```cmd
$ solana config set -u devnet
$ solana config set -k <FULL_PATH_TO_KEYPAIR.JSON>
```

# Update Packages

Rust
```cmd
$ rustup update
```

Solana (check [here](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool) for latest version)
```cmd
$ sh -c "$(curl -sSfL https://release.solana.com/v1.14.18/install)"
```

# Build Program

Creates a keypair (program account) and shared object (binary) file in the `target/deploy/` folder.
```cmd
$ cd <the program directory>
$ cargo build-sbf
```

# Deploy Program

Deploys the program's binary to the cluster and returns the `program id`.
```cmd
$ solana program deploy target/deploy/boq.so
```

# Debugging

Outputs msg! calls
```cmd
$ solana logs
```

View transaction logs
```cmd
$ solana confirm -v <TRANSACTION_HASH>
```