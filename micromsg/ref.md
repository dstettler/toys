# MicroMsg Reference
This document includes general information for me to reference when writing the implementation code of this messaging protocol.
It will be structured in a few major subjects you can jump to:
- [Requirements](#general-functionality-requirements)
- [Data Structures and Storage](#data-structures-and-storage)
- [Protocol Reference](#protocol-reference)

Unless otherwise specified, all connections between client and broker should be TLS, and all client-client connections TCP.

# Requirements
## General Functionality Requirements
- Users can register with broker, get number.
- Users can contact other users via their number.
- Users can communicate on multiple devices.
- Users can communicate with multiple users in a group chat format.
- Messages can be encrypted end-to-end.
- Encryption keys can be synched across user devices.
- Users can read any message from any synched device.
- Brokers can store encrypted messages until user has an online device.
- Brokers can share a number space in a pre-contracted registered network.
- Brokers can send messages to other brokers in their registered network.
- Clients should send intermittent heartbeats to each other to maintain a session.
- Clients should send a message indicating they are disconnecting to end a session.
- Clients should handle group messaging locally

# Data Structures and Storage
## Client Keys
List of keys that must be stored for every client.
- Identity keypair: Asymmetric keypair to verify identity at start of each online session.
Can be recycled at-will, or at a specified interval (organized via broker). Each client sends the other their
pubkey, and stores their personal privkey, and target pubkey.
- Session key seeds: Every online session will have a symmetrically encrypted key seed. Each message will be encrypted
using a derived symmetric key based on this seed. Timestamps used to derive these keys are included in the message header, and
can be decrypted via the per-session timestamp key.
- Timestamp keys: Ephemeral asymmetric keypair generated at the start of the session. Each message will be headered by a UNIX
timestamp[^1] that is encrypted with these keys. This timestamp is then used to derive the per-message symmetric key.
- Broker keypairs (optional): If the client is in a multi-broker network, it will generate an asymmetric keypair for communicating directly
with any broker that is not their 'home'. More in [Inter-Broker Communication](#inter-broker-communication).

[^1]: Since per-message keys are based on UNIX timestamps, there may be multiple messages sent with the same given time.
To avoid conflicts, each timestamp may optionally be prepended with a `#-` counting up from 0. This requires treating these timestamps as strings,
although if network efficiency of a message is a concern, this can be changed.

# Protocol Reference
## Client-Broker New Registration
1. Client connects to broker, requests new entity, gets client/entity number and client secret for future client->broker auth.
2. Client returns its own server secret which is used to verify the broker is who they say they are (beyond TLS).
Client/server secret is paired with UNIX timestamp to use as base for authentication.
Every 6 seconds (or time specified by broker) new nonce is generated based on the seed according to a specified nonce algorithm.
3. Registration is now complete. Client must log in using the login protocol to make any actual requests.

Note: Since registrations are likely to be abused, there are a few recourses a broker may take. They may require an optional `TOKEN` argument which can be checked before providing a new entity number and secrets.

## Client-Broker Login
1. Client connects to broker, requests login with its client/entity number and provides secret-encrypted nonce.
2. If previous step is valid, server responds with its own secret-encrypted nonce to verify its identity.
3. Client requests whatever it needs.

## Client-Broker Device Registration
1. Existing client logs into broker.
2. Client requests broker add new device to entity.
3. Broker gives client a registration secret.
4. New client connects to broker, requests new device registration

## Client-Client Setup (same broker)
1. Client requests connection to number from broker.
2. Broker sends an ACK and checks if number is available
    - If number unavailable, *do not respond.* 
    Optionally, a broker may save this invalid request to a blacklist,
    and future clients given that number will not be contactable by this client.
3. Send connection request to online client at specified number.
    - Similar to above, if client rejects, add contacter to blacklist.
4. When connection request is accepted, send IP to contacter for peer-to-peer key exchange.
5. Perform peer-to-peer key exchange.
6. Optionally, clients may allow the broker to hold offline messages for them by establishing themselves as a brokered contact pair.
They may also allow the server to immediately send contacts their IP to establish a connection once their identity is verified, rather than asking.

## Client-Client Communication (both online, one device each, same broker)
1. Client connects to IP:port given by server, sends nonce for verification.
2. If nonce is valid, client responds with its own nonce.
3. Establish timestamp keypair.
4. Establish per-message key seed.
5. For every message sent:
    - Encrypt UNIX timestamp w/ timestamp key.
    - Derive message key from timestamp (and optionally message number).
    - Append timestamp to header.
6. When client goes offline, send a notice.

## Client-Client Communication (one offline, one device each, same broker)
1. Client uses previous session key
2. If nonce is valid, client responds with its own nonce.
3. Establish timestamp keypair.
4. Establish per-message key seed.
5. For every message sent:
    - Encrypt UNIX timestamp w/ timestamp key.
    - Derive message key from timestamp (and optionally message number).
    - Append timestamp to header.

## Handling Multiple Clients
Users may want to have multiple receiving clients-
in introducing this additional layer of complexity, the 1:1 nature of realtime communication breaks down and tradeoffs need to be made.

With respect to offline message queueing, smaller brokers with known device pools may want to copy messages to every device's 'inbox' to avoid
synchronization issues associated with keeping the onus on end devices.
In contrast, incredibly large brokers would likely wish to avoid the excess storage this approach would require,
and as such may wish to push this responsibility onto the clients.
More on this in [Entity Configuration](#entity-configuration) and [Number Configuration](#number-configuration).

Additionally, keeping keys synched, clients should be given the option of storing key data with their broker using known symmetric key encryption,
or to keep things point-to-point using end-to-end negotiated keys.

Brokers should make transparent what configurations they use,
and who they can communicate with (more in [Inter-Broker Communication](#inter-broker-communication)),
so clients can make informed decisions before registering their number with that broker.

Additionally, there should be systems in place to allow users to take data out of one broker and move to a new one while retaining previous connection data.
This may be performed client-side, or through a broker-level system.

### Entity Configuration
In entity configuration, each number is treated as an 'entity' with multiple saved client devices.
When establishing a communication channel between two entities, the broker informs the contacting it is operating in this mode, giving it the 
addresses of the top online contactee (how this is determined is up to implementation), 
as well as requesting a copy that will be cloned for each other device in the entity.

Timestamp and seed keys can be synched either via aforementioned client-encrypted broker storage, or directly sent between devices as needed, depending on the
client setup.
 

### Number Configuration

## Inter-Broker Communication
An optional setup for a broker may also be in multi-broker mode.
This presents a security challenge in that client IP addresses will need to travel between double the amount of hands (2 instead of 1),
but this can be mitigated slightly by making use of a broker-level identity key.

When a client attempts to first make a connection to another client in the broker network, those brokers will each request an public key from their respective clients
and hand these off each other. 
They will then use these public keys to encrypt the addresses of their online clients to ensure the only points of failure are the two clients, and the home brokers
themselves.

The brokers will store pubkeys for each contacted client for as long as they are in their network. It is important these are given
expiry times to avoid wasting too much storage on these.

In this configuration, a number pool is distributed across brokers (which must all remain online), where each broker has a few options
for saved messages. The configurations will be explained as follows.

## Broker Requests
After establishing a TLS connection with a broker, a client may make certain requests.
These requests are often *stateful* and as such require a persistent connection.

- `REGISTER`[^2]: Client requesting a new entity number. Return data [as specified here](#client-broker-new-registration).
- `ADD-DEVICE`[^2]: Client requesting an add device flow [as specified here](#client-broker-device-registration).
Client *must* be authenticated from a previous login request.
- `LOGIN`: Client logs in using pre-determined secret [as specified here](#client-broker-login).
Args:
    - `SECRET`: Encrypted nonce
    - `ENTITY`: Entity number used to log in
- `INITIATE-NEW`: Client requests connection to new contact [as specified here](#client-client-setup-same-broker).
Args:
    - `TARGET`: Number in this broker's number space.
- `INITIATE-NEW-NETWORKED`[^2]: Client requests connection to new contact that located at a different broker [as specified here](#client-client-setup-different-broker).
Args:
    - `TARGET`: Number in this broker's number space.
- `INITIATE-EXISTING`: 
Args:
    - `TARGET`: Number in this broker's number space.
- `INITIATE-EXISTING-NETWORKED`[^2]: 
Args:
    - `TARGET`: Number in this broker's number space.
- `IS-NETWORKED`[^2]: Client asks broker if the given number is in their share of the number space. *This does not necessarily mean the number is valid.*
Args:
    - `TARGET`: Number in this broker's number space.

### Request Format
All requests and responses will be YAML encoded. An example with a login request:

```yml
Request: LOGIN
Args:
    TARGET: 
```

# Limitations
There are several limitations of this protocol compared to more advanced and developed protocols like [Signal's](https://en.wikipedia.org/wiki/Signal_Protocol).
For one, in order to initially establish a session and create inboxes, two clients *must* be online at the same time, and as such
this protocol will not function in asynchronous environments similar to that of Signal.

Additionally, 