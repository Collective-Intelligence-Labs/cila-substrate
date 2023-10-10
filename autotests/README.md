# Understanding the Code

The tests are organized in a JavaScript file. Below are the key components and steps performed in the code:

## Importing Required Libraries:
The `@polkadot/api` and `protobufjs` libraries are imported to interact with the Substrate node and handle Protocol Buffers, respectively.

```javascript
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const protobuf = require('protobufjs');
```

## Loading Protobuf Schema:
The compiled protobuf schema is loaded, and the `Command` message type is looked up.

```javascript
const root = protobuf.loadSync('../proto/src/command.proto');
const Command = root.lookupType("Command");
```

## Setting Up Connection:
A WebSocket provider is set up, and the API connection to the Substrate node is established.

```javascript
wsProvider = new WsProvider('ws://127.0.0.1:9944');
api = await ApiPromise.create({ provider: wsProvider });

## Listing Available Pallets and Methods:
The available pallets and their methods are logged to the console.

```javascript
listAllPalletsAndMethods();
```

## Sending Commands:
Functions for sending commands and interacting with the blockchain are defined.

```javascript
function sendCommand(action, params) { /*...*/ }
```

## Running Tests:
Tests are defined and run using Jest's `describe` and `test` functions.

```javascript
describe('Substrate Tests', () => { /*...*/ });
```

## Cleaning Up:
The connection to the Substrate node is closed after the tests have run.

```javascript
afterAll(() => {
  return api.disconnect();
});
```