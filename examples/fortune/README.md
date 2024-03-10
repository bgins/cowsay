# fortune

This example runs the Cowsay Wasm component in a [Homestar][homestar] workflow. The workflow is injected with a fortune produced at the command line, then run on the Homestar runtime.

The fortunes are borrowed from the [`fortune-kind` collection][fortune-kind-collection]. Please see the [`fortune-kind` licenses][fortune-kind-licenses] for reuse of `fortune-kind` and fortunes.

## Setup

[Install Rust][install-rust], then install [`fortune-kind`][fortune-kind] using Cargo:

```sh
cargo install fortune-kind
```

Install [IPFS Kubo][ipfs-kubo].

## Use

### Compile the Wasm component

Compile a release build as described in the [top-level README][top-level-readme].

### Add the Wasm component to IPFS

Start the IPFS daemon:

```sh
ipfs daemon
```

Add the component to IPFS:

```sh
ipfs add --cid-version 1 ../../target/wasm32-wasi/release/cowsay.wasm
```

Check that the CID reported by IPFS matches the `rsc` field in `workflow-template.json`. If not, update `workflow-template.json` with the reported CID.

### Generate a workflow

Run the script to create a fortune and insert it into a workflow:

```sh
./generate.sh
```

### Run the workflow

Start Homestar:

```sh
npx homestar-runtime start
```

Run workflow in another terminal window:

```sh
npx homestar-runtime run workflow.json
```

Run the workflow a second time, and copy the replayed receipt CID into:

```sh
ipfs dag get <replayed-receipt> | jq ."out[1]" --raw-output
```

This command should print the cow saying a fortune.

[fortune-kind]: https://crates.io/crates/fortune-kind
[fortune-kind-collection]: https://github.com/cafkafk/fortune-kind/tree/main/fortunes
[fortune-kind-licenses]: https://github.com/cafkafk/fortune-kind/tree/main/LICENSES
[homestar]: https://github.com/ipvm-wg/homestar
[ipfs-kubo]: https://docs.ipfs.tech/install/command-line/#install-official-binary-distributions
[install-rust]: https://www.rust-lang.org/tools/install
[top-level-readme]: ../../README.md
