# Places - NEAR Contract

A smart contract in the NEAR network to store places and enable users to vote and set their qualities.

<br />

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)

<br />

## 1. Build, Test and Deploy

To build the contract you can execute the `./build.sh` script, which will in turn run:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Then, run the `./deploy.sh` script, which will in turn run:

```bash
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/hello_near.wasm
```

the command [`near dev-deploy`](https://docs.near.org/tools/near-cli#near-dev-deploy) automatically creates an account in the NEAR testnet, and deploys the compiled contract on it.

Once finished, check the `./neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1659899566943-21539992274727
```

<br />

## 2. Write Methods [payable]

```rs
// Add Place
pub fn add_place(&mut self, place: PlaceInput)

// Vote
pub fn vote(&mut self, place_id: u64, vote: i8)

// Add pictures to a Place
pub fn add_picture_to_place(&mut self, place_id: u64, pictures: Vec<String>)

// Remove a place
pub fn remove_place(&mut self, place_id: u64)
```

## 3. Read Methods

```rs
// Returns the Places
pub fn get_places(&self) -> Vec<Place>

// Get places by its id
pub fn get_places_by_id(&self, place_id: u64) -> Option<Place>
```

## 4. Retrieve the Greeting

`get_places` and `get_places_by_id` is a read-only method (aka `view` method).

`View` methods can be called for **free** by anyone, even people **without a NEAR account**!

```bash
# Use near-cli to get the places
near view <dev-account> get_places
```

<br />

## 5. Store a New Greeting

`add_place`, `vote`, `add_picture_to_place` and `remove_place` changes the contract's state, for which it is a `change` method.

`Change` methods can only be invoked using a NEAR account, since the account needs to pay GAS for the transaction. In this case, we are asking the account we created in step 1 to sign the transaction.

```bash
# Use near-cli to add a new place
near call <dev-account> add_place '{"place":{"name": "Natura Store","address": "Pampulha","description": "A place to buy perfume.","pictures": ["https://lh5.googleusercontent.com/p/AF1QipMBMUOyXp7E1gZRB_KVeKLOLOpZv1bzZt-JxsAd=w408-h306-k-no"]}}' --accountId <dev-account>
```

**Tip:** If you would like to call `set_greeting` using your own account, first login into NEAR using:

```bash
# Use near-cli to login your NEAR account
near login
```

and then use the logged account to sign the transaction: `--accountId <your-account>`.
