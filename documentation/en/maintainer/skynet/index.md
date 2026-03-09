# Skynet Protocol

Skynet is AI Company's decentralized communication protocol, designed based on a zero-trust architecture to enable secure, privacy-preserving instant messaging networks.

## Protocol Scope

The Skynet Protocol is **responsible for**:
- Node identity authentication and network discovery
- End-to-end encrypted communication (X3DH + Double Ratchet for private chats, MLS for group chats)
- Subnet data storage and synchronization (Byzantine fault-tolerant replication)
- Data structure definitions for messages, resources, and user metadata

The Skynet Protocol is **not responsible for**:
- Key management (storage, backup, recovery) - the client's responsibility
- Random number generation and entropy sources - the client's responsibility
- Post-quantum cryptography - out of scope for now
- Client security mechanisms such as hardware security modules, biometrics, etc. - the client's responsibility

See [Protocol Boundaries and Disclaimers](./skynet.md#10-协议边界与免责声明) for more details.

## Core Concepts

### 1. Core Identifiers
The Skynet Protocol has three core identifiers:
- **auth_id**: Global user identity ID, unique across subnets, similar to a real name. **The top secret of the Skynet Protocol, used only for identity authentication, and must never be leaked or stored in subnet internal data.**
- **user_id**: User's local ID within a subnet, similar to a nickname/online name, used for privacy protection. **All data within a subnet uses only user_id.**
- **subnet_id**: Unique subnet ID, identifying an independent communication system

### 2. Subnet Model
Skynet adopts a subnet model, where each independent communication system or community is a subnet:
- Each subnet has its own users, contacts, and groups
- Users can join multiple subnets simultaneously using the same `auth_id`
- Each subnet has independent contacts and groups (contacts are an internal subnet concept)

### 3. Decentralized Architecture
Skynet uses a two-tier network structure:
- **Mainnet**: Peer-to-peer network composed of service nodes, responsible for node discovery and network routing
- **Subnets**: Independent communication systems, isolating data from different platforms

### 4. Service Nodes
- Each node has an independent Ed25519 key pair and unique NodeID
- Nodes use Kademlia DHT for network discovery and address storage
- Supports STUN/TURN protocols for NAT traversal

### 5. Subnets
Subnets are independent communication systems:
- Each subnet has a unique subnet_id
- A set of maintenance nodes are responsible for data storage and synchronization
- Supports internal subnet functions such as member management, channels, etc.

### 6. Resources
Resources are accessible entities within a subnet:
- Resources have a unique resource ID (resource_id)
- Resources can be files, images, documents, links, etc.
- Resources can be authorized for access and management
- Supports resource version control and history records
- **Not shared across subnets**: Resources are only accessible within their belonging subnet, and must be copied if needed for cross-subnet use

### 7. Messages
Messages are content units transmitted within a subnet:
- Messages have a unique message ID (message_id)
- Message content is **structured data** (JSON objects), not fixed Markdown or HTML format
- Clients independently decide how to render based on message type and content structure
- Messages support end-to-end encryption
- Messages can be sent in channels or private chats
- **Not transmitted across subnets**: Messages do not cross subnets, and must be copied to the target subnet if forwarding is needed
- **Optional message status**: Supports delivery tracking such as delivered, read, etc. (optional extension)

### 8. Message Reactions
Message reactions are emotional feedback from users on messages:
- Users can add emoji reactions to messages (e.g., 👍, ❤️, 🎉, etc.)
- Supports custom emojis
- Can view which users added which emoji
- Can revoke reactions they added

### 9. Message Pinning
Message pinning is used to mark important messages:
- Can pin important messages to the top of channels or private chats
- Supports multiple message pins
- Pinned messages can be sorted by time or manually
- Only administrators or message senders can pin (configurable)

### 10. Message Threads
Message threads are used for in-depth discussions under messages:
- Can create reply threads under any message
- Thread messages are displayed independently and do not affect the main chat flow
- Supports message notifications within threads
- Threads can be collapsed/expanded

### 11. Message Mentions
Message mentions are used to notify specific users:
- **@User**: Mentions a specific user, that user will receive a notification
- **@Everyone**: Mentions all members of a channel
- **@Role**: Mentions all members of a specific role (e.g., @Admin)
- Mentioned messages will be highlighted

### 12. User Profiles
User profiles describe a user's public information:
- Avatar: User avatar image or hash
- Nickname: User's display name within a subnet
- Bio: User's personal introduction
- Custom status text: User's custom status text
- Presence status: User's online status (online/busy/away/offline)
- Last active at: Time when the user was last active
- Profiles are public and stored in the subnet member list

### 13. Zero-Trust Security
- All critical operations require cryptographic signature verification
- No trust between service nodes, or between clients and servers
- End-to-end encryption ensures only communicating parties can decrypt content

## Documentation Navigation

| Document | Description |
|----------|-------------|
| [Skynet Protocol Design Draft](./skynet.md) | Complete technical design documentation for the Skynet Protocol, including node identities, user authentication, end-to-end encryption, data storage and synchronization, etc. |
| [Threat Model](./threat-model.md) | Analyzes security threats and mitigation measures for the Skynet Protocol using the STRIDE framework |
| [URI Protocol](./uri.md) | Skynet URI protocol specification, defining the format and usage of the skynet:// protocol header |
| [Subnet Model](./subnets.md) | Specific implementation details of subnets, including subnet identification, data structures, maintenance nodes, etc. |
| [Resource Model](./resources.md) | Specific implementation details of resources, including resource types, storage, permissions, etc. |
| [Message Model](./messages.md) | Specific implementation details of messages, including message types, formats, statuses, reactions, pinning, threads, mentions, etc. |
| [User Profiles](./profile.md) | Specific implementation details of user profiles, including avatars, nicknames, online status, etc. |

## Relationship to Architecture Design

The Skynet Protocol is the concrete technical implementation of the **decentralized design concept** in AI Company's [architecture design](../architecture/index.md).

### Relationship Diagram

```
Decentralized Design Concept (architecture/)
    ↓ guides
Skynet Protocol Implementation (skynet/)
    ↓ implements
Actual System Operation
```

### Specific Correspondence

| Design Concept | Skynet Implementation |
|----------------|----------------------|
| Zero-trust architecture | auth_id authentication, end-to-end encryption, mutual distrust assumption |
| Decentralization | Peer-to-peer network, mainnet-subnet two-tier structure, no single point of failure |
| Verifiable computation | Digital signatures, multi-node cross-validation, Byzantine fault tolerance |
| Data sovereignty | End-to-end encryption, user-controlled keys, servers cannot access plaintext |

See [Architecture Design](../architecture/index.md) for design concepts.
