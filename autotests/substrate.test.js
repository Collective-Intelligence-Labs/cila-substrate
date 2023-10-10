const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const protobuf = require('protobufjs');

// Load the compiled protobuf schema
const root = protobuf.loadSync('../proto/src/command.proto');
const Command = root.lookupType("Command");


let api;
let wsProvider;

// Function to create and send a command
function sendCommand(action, params) {
  // Create a new Command message
  const command = Command.create({ action, params });

  // Encode the command to a buffer
  const buffer = Command.encode(command).finish();
  const commandBytes = buffer; // new Uint8Array([/*...*/]);

  // Create and sign the extrinsic
  // Replace 'your_account' and 'your_private_key' with your actual account and private key
   // Create a keyring instance
  const keyring = new Keyring({ type: 'sr25519' });

   // Add Alice to the keyring (using a known/public seed for Alice)
  const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

  // Log the transaction hash
  const extrinsic = api.tx.dispatcher.dispatch(commandBytes);

  // Sign and send the extrinsic
  return extrinsic.signAndSend(alice);
}

function listAllPalletsAndMethods(){
  // Get the list of available pallets
  const pallets = Object.keys(api.tx);

  // Log the available pallets and their methods
  pallets.forEach(pallet => {
    console.log(`Pallet: ${pallet}`);
    const methods = Object.keys(api.tx[pallet]);
    console.log('Methods:', methods.join(', '));
  });

}

beforeAll(async () => {
  wsProvider = new WsProvider('ws://127.0.0.1:9944');
  api = await ApiPromise.create({ provider: wsProvider });
  listAllPalletsAndMethods();
  
   // Create a keyring instance
   const keyring = new Keyring({ type: 'sr25519' });

   // Add Alice to the keyring (using a known/public seed for Alice)
   const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

  // Create the extrinsic for the addRouter function
  // Replace 'YourPalletName' with the name of your pallet
  // Replace 'router_address' and 'router_info' with the actual values
  const extrinsic = api.tx.dispatcher.addRouter(alice.publicKey);

  // Sign and send the extrinsic using Alice's account
  const hash = await extrinsic.signAndSend(alice);

  // Log the transaction hash
  console.log('Transaction hash:', hash.toHex());
});

afterAll(() => {
  return api.disconnect();
});

describe('Substrate Tests', () => {
  
  test('Send Mint NFT transaction', async () => {
    var result = await sendCommand('MintNFTPayload', {hash: "asdasd", owner: "asasdsad"});
    expect(result);
  });

  test('Add router', async () => {
    const keyring = new Keyring({ type: 'sr25519' });
    const bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
    const extrinsic = api.tx.dispatcher.addRouter(bob.publicKey);
    const hash = await extrinsic.signAndSend(bob);
    expect(hash);
  });

  // ... add more tests as needed
  
});