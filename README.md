# Places - NEAR Contract

A smart contract in the NEAR network. This contract allows uses to register places around the world, evaluate them, post feedback and consult them.

Access the live dapp here:

- [**Places DApp (mainnet)**](https://places-dapp-near.vercel.app/)
- [**Places DApp (testnet)**](https://places-dapp-near-testnet.vercel.app/)

Frontend repository: [**places-near-dapp**](https://github.com/wpdas/places-near-dapp)

<br />

# General Info

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
// Init contract - called by-owner only (determines who can do admin actions)
pub fn init(owner: AccountId, admins: Option<Vec<AccountId>>)

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

## 4. Retrieve the Places

`get_places` and `get_places_by_id` is a read-only method (aka `view` method).

`View` methods can be called for **free** by anyone, even people **without a NEAR account**!

```bash
# Use near-cli to get the places
near view <contract-id> get_places
```

<br />

## 5. Store a New Place

`add_place`, `vote`, `add_picture_to_place` and `remove_place` changes the contract's state, for which it is a `change` method.

`Change` methods can only be invoked using a NEAR account, since the account needs to pay GAS for the transaction. In this case, we are asking the account we created in step 1 to sign the transaction.

```bash
# Use near-cli to add a new place
near call <contract-id> add_place '{"place":{"name": "Natura Store","address": "Pampulha","description": "A place to buy perfume.","pictures": ["https://lh5.googleusercontent.com/p/AF1QipMBMUOyXp7E1gZRB_KVeKLOLOpZv1bzZt-JxsAd=w408-h306-k-no"]}}' --accountId <user-account-id>
```

**Tip:** If you would like to call `add_place` using your own account, first login into NEAR using:

```bash
# Use near-cli to login your NEAR account
near login
```

and then use the logged account to sign the transaction: `--accountId <your-account>`.

<br />

## 6. View Contract State with Postman

You can see the contract state by sending a POST to `https://rpc.testnet.near.org` with the following body:

```json
{
  "jsonrpc": "2.0",
  "id": "dontcare",
  "method": "query",
  "params": {
    "request_type": "view_state",
    "finality": "final",
    "account_id": "<contract-id>",
    "prefix_base64": ""
  }
}
```

<br />

## 7. Deploying to Mainnet (Production)

You can go to [NEAR CLI](https://docs.near.org/tools/near-cli) page to understand more. But below, are some steps to deploy the contract for Mainnet.

Change the network by prepending an environment variable to the commands.

After building your contract:

1. Login with your account:

```sh
NEAR_ENV=mainnet near login
```

2. Create an account/sub-account to store the Contract

```sh
# In this case, you'll use your credentials
NEAR_ENV=mainnet near create-account <created-account-id>.wendersonpires.near --masterAccount wendersonpires.near --initialBalance <amount-of-near-to-send-to-the-new-account-being-created>

# e.g NEAR_ENV=mainnet near create-account place-contract.wendersonpires.near --masterAccount wendersonpires.near --initialBalance 2
```

3. Deploy Contract to the created account

```sh
NEAR_ENV=mainnet near deploy --accountId <created-account-id>.wendersonpires.near --wasmFile ./target/wasm32-unknown-unknown/release/places_near_contract.wasm

# e.g NEAR_ENV=mainnet near deploy --accountId place-contract.wendersonpires.near --wasmFile ./target/wasm32-unknown-unknown/release/places_near_contract.wasm
```

4. Now, the Contract is ready to be used

```sh
NEAR_ENV=mainnet near view place-contract.wendersonpires.near get_places
```

<br />

## 8. Contract Address

### Mainnet

```
place-contract.wendersonpires.near
```

### Testnet

```
place-contract.wendersonpires.testnet
```

## 9. Good to know

### Mainnet

Account `place-contract.wendersonpires.near` created with:

```sh
NEAR_ENV=mainnet near create-account place-contract.wendersonpires.near --masterAccount wendersonpires.near --initialBalance 2
```

Contract deployment using account id `place-contract.wendersonpires.near`:

```sh
NEAR_ENV=mainnet near deploy --accountId place-contract.wendersonpires.near --wasmFile ./target/wasm32-unknown-unknown/release/places_near_contract.wasm
#Transaction: https://explorer.mainnet.near.org/transactions/BdX7Dyr3wKcTBmM14jyNfcrt344gN5sRexJweVdioLfr
```

Contract initialized with:

```sh
NEAR_ENV=mainnet near call place-contract.wendersonpires.near init '{"owner": "wendersonpires.near"}' --accountId wendersonpires.near
# Transaction: https://explorer.mainnet.near.org/transactions/GaTRTxFzGAHMU6PpypAvyARkForfRhhZJy2pdsawpyGg
```
