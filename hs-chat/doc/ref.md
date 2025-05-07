# HS-Chat
Learning how to use Haskell with a simple chat app, since I don't remember much from my type inferencer. :)

## General Architecture
Each client will estbalish a connection with another client, and request to speak with a user id.
If a previous conversation has already been established, they will also send an agreed-upon secret using the other side's public key,
and will be given the same in turn.
Then, they will agree upon a session AES key.

Otherwise, generate RSA keys, encrypt with a known shared secret, and pass pubkeys between clients before using the same AES share.
Think of the handshake like a very simplistic TLS.
This is not incredibly high-security (far from the signal protocol), but is enough for a simple verification of sender-receiver.

With the simplicity of the message architecture, centralized servers can act as an escrow and pass messages between authenticated clients to,
for example, protect IP addresses, etc.

The session key will be used to derive message keys, with an unencrypted hash of the 'message number' being included in the message.
This allows clients to quickly verify which message key to derive in order to decrypt messages, in the event of several messages being sent
in rapid succession.

Once a session has been established, and since messages are P2P, sender and receiver ids are not necessary to include.

## Message Structure
Messages will be structured similar to HTTP, with content delimited by newlines, and as a list of key-value pairs delimeted by a colon.
All messages include an HSC version and a type in their first line, and will end with a line containing only: `END-HSC`
### Authentication
#### New Handshake (Initiator -> Receiver)
```
HSCv1 new-handshake
Sender: <sender-id>
Key: <receiver user id>,<pubkey> # Note that this will be encryptedd
Hash: <hash of decrypted field>
Hash-Ver: <version of hashes available to use>
Cipher-Ver: <ciphers available to use>
```

#### New Handshake Return (Receiver -> Initiator)
```
HSCv1 new-handshake-return
Key: <receiver user id>,<pubkey> # Note that this will be encryptedd
Hash: <hash of decrypted field>
Hash-Ver: <version of hashes to available use>
Cipher-Ver: <ciphers available to use>
```

#### Existing Handshake (Initiator -> Receiver)
```
HSCv1 existing-handshake
Sender: <sender-id>
Key: <previous secret encrypted with pubkey>
Hash-Ver: <version of hashes to available use>
Cipher-Ver: <ciphers available to use>
```

#### Existing Handshake Return (Receiver -> Initiator)
```
HSCv1 existing-handshake-return
Key: <previous secret encrypted with pubkey>
Hash-Ver: <version of hashes to available use>
Cipher-Ver: <ciphers available to use>
```

#### Handshake Session Key (Initiator -> Receiver)
```
HSCv1 handshake-session
Key: <session key encrypted with pubkey>
Sel-Hash: <selected hash type>
Sel-Cipher: <selected cipher type>
```

### Messaging
#### Forwarded Message (i.e. for Initiator -> Escrow -> ... -> Receiver)
```
HSCv1 forward
Sender: <sender id>
Endpoint: <receiver id>
Message: <message content, with END-HSC line included>
Should-Cache: <True/False>
```

#### Simple Message
```
HSCv1 msg
Message: [msg]<message contents>[/msg]
```

