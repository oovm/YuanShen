# Skynet Protocol Design Draft

## 1. Overview
Skynet is a decentralized communication protocol designed to build a secure, privacy-preserving instant messaging network. Its core architecture consists of **service nodes** forming a peer-to-peer network that maintains a two-level structure of **main network** and **subnetworks**. Clients can connect to any service node and establish secure communication between devices through end-to-end encryption. The protocol design follows the **zero-trust** principle: both between service nodes and between service nodes and clients, each party assumes the other may be malicious, and all critical data must be cryptographically verified.

### 2. Node Identity and Network Discovery
- **Node Identifier**: Each service node has a long-term Ed25519 key pair, with the Node ID being a hash of the public key (`NodeID = blake3(public_key)`). Inter-node communication uses the Noise protocol (XX pattern) to establish encrypted channels and mutually verify NodeIDs.
- **Network Discovery**:
  - Initial nodes join the network through a pre-configured list of seed nodes.
  - Nodes maintain a Kademlia-style Distributed Hash Table (DHT) to store address information (IPv4/IPv6 + port) of other nodes and perform liveness probing.
  - Nodes periodically publish their addresses to the DHT and refresh their neighbor lists.
  - Each node must support STUN protocol for NAT traversal assistance; when traversal is not possible, relaying through other nodes is allowed (TURN-like functionality, but relay nodes only forward encrypted data).

### 4. User Identity and Authentication
- **Global Identity**: Each user has a global UUID `auth_id` and a long-term Ed25519 key pair (for identity signing). This identity is generated when the user first uses the system, and the public key is signed and published by the user. **auth_id is the highest secret of the Skynet protocol and must never be leaked or stored in subnetwork internal data.**
- **Subnetwork Model**:
  - Each independent communication system or community is a separate subnetwork
  - Users have different contact lists in different subnetworks
  - `auth_id` is like a real name, uniquely identifying users across subnetworks, **used only for user identity authentication and not used in subnetwork internal data**
  - `user_id` is like a nickname/screen name, a local identifier within the subnetwork for privacy protection, **all data within the subnetwork only uses user_id**
  - Users only use `user_id` as their identifier within a subnetwork
  - The user's public key within the subnetwork (can use the global identity public key or separately generate a subnetwork-specific key) must be signed by themselves and submitted to subnetwork maintenance nodes
  - Users can join multiple subnetworks simultaneously, authenticating with the same `auth_id`
  - **Subnetwork Type**: Each subnetwork has a type identifier (e.g., `organization`, `community`, `private`, `custom`), and the client decides whether to load and how to process based on the type
  - **Subnetwork Metadata**: Each subnetwork contains metadata (type, version, name, description, features, etc.) for client identification and adaptation
- **Client Authentication**:
  - When a client connects to a service node, it must provide `auth_id` and complete a challenge-response: the service node sends a random number, the client signs it with its identity private key and returns it, and the service node verifies the signature.
  - After successful authentication, the service node generates a temporary session key for the client (for subsequent request encryption) and maintains the client's online status during the session validity period.

### 5. End-to-End Encryption
All inter-user communication content (private messages, group chat messages, file transfers) uses end-to-end encryption, and service nodes cannot decrypt it. The encryption scheme draws inspiration from the Signal protocol and MLS protocol:
- **Device Keys**: Each client device generates an X25519 key pair as its device identifier, signs it with the user's identity private key, and publishes it to the subnetworks it has joined (as part of the member list). A user may have multiple devices.
- **Private Chat**:
  - Both parties establish a shared key through the X3DH key exchange protocol and use the Double Ratchet algorithm to generate session keys, achieving forward secrecy and post-compromise security.
  - Initial key exchange messages can be forwarded (encrypted) through service nodes, which only transmit ciphertext.
- **Group Chat**:
  - Uses the MLS (Messaging Layer Security) protocol, providing forward secrecy, post-compromise security, and efficient key rotation when membership changes.
  - The MLS protocol automatically handles key updates when members join/leave without requiring manual administrator intervention.
- **Metadata Protection**:
  - Service nodes can only see the message recipients (target subnetwork and user/group), but cannot see the message content. To hide recipients, mix network (e.g., Loopix) technology may be considered, but it is not implemented in the initial version and is left for future expansion.

### 5.1 Security Parameters (Efficiency Priority)
The Skynet protocol adopts the following security parameters, prioritizing performance and user experience while ensuring enterprise-grade security:
| Parameter | Value | Description |
|-----------|-------|-------------|
| **Signature Algorithm** | Ed25519 | Node and user identity signatures |
| **Key Exchange** | X25519 | Used by X3DH and MLS |
| **Hash Algorithm** | Blake3-256 | 256-bit output, used for NodeID, content_hash, etc. |
| **Symmetric Encryption** | ChaCha20-Poly1305 | Message encryption, 256-bit key |
| **Noise Protocol** | Noise_XX_25519_ChaChaPoly_BLAKE2s | Inter-node communication |
| **X3DH Pre-key Count** | 100 | Number of pre-generated one-time pre-keys |
| **MLS Cipher Suite** | MLS10_128_HPKEX25519_AES128GCM_SHA256_Ed25519 | MLS protocol cipher suite |

### 5.2 Design Principles
The Skynet protocol follows these core design principles:
1. **Efficiency Priority**: Prioritize system performance and user experience while ensuring basic security
2. **Modern Solutions**: Adopt well-proven modern cryptographic primitives and protocols
3. **Extensible Design**: Architecture supports smooth expansion of future security mechanisms
4. **Practical Security**: Provide enterprise-grade security guarantees without pursuing over-design for military-grade security

### 5.3 Replay Attack Protection
All encrypted messages include anti-replay mechanisms:
- **Message Sequence Number**: Each message contains a monotonically increasing 64-bit unsigned integer sequence number
- **Replay Window**: The receiver maintains a sliding window of the most recent 1000 message sequence numbers
- **Verification Logic**:
  1. If the sequence number ≤ window minimum, discard
  2. If the sequence number is within the window and already exists, discard
  3. Otherwise accept the message and update the window
- **Server-side Protection**: Maintenance nodes also verify sequence numbers to prevent malicious clients from replaying messages

### 6. Data Storage and Synchronization
Subnetwork data is jointly stored by a set of maintenance nodes, ensuring reliability and consistency even when some nodes are malicious.
- **Data Model**:
  - All subnetwork data is stored in the form of **immutable operation logs**, with each operation (e.g., adding a member, creating a resource, sending a message) signed by the initiator and containing a monotonically increasing sequence number.
  - The log eventually converges to a deterministic state (can be reconstructed by replaying operations).
  - **Resource Data**: Includes metadata and content hashes of resources such as files and images; resource content can be stored in distributed storage (e.g., IPFS) or local storage.
  - **Message Data**: Includes message content, sender, receiver, timestamp, etc., supporting end-to-end encryption.
- **Replication Protocol**:
  - Maintenance nodes exchange operation logs through the Gossip protocol, with each node independently verifying signatures and permissions.
  - Uses **vector clocks** or **version vectors** to detect conflicts; conflict resolution rules are defined by subnetwork policies (e.g., "administrator operation priority" or "timestamp priority").
  - Nodes periodically take hash snapshots of the log and compare them with other nodes; if inconsistencies are found (e.g., a node maliciously tampering with history), other nodes can mark it as untrusted and trigger administrator intervention.
- **Write Operations**:
  - The client submits a write operation (e.g., inviting a member) to any service node, which forwards the request to all maintenance nodes.
  - After verifying permissions and signatures, maintenance nodes append the operation to their local logs and broadcast to other maintenance nodes.
  - The client can require confirmation from a majority of maintenance nodes before considering the operation successful, or use "read-after-write" verification: read the latest state from multiple maintenance nodes and alert if inconsistent.
- **Read Operations**:
  - The client can read subnetwork data from any maintenance node, but must obtain and compare from multiple nodes, adopting the majority consensus result; or only trust data signed by enough nodes (e.g., aggregated signatures).
  - To improve performance, the client can cache data and periodically poll for updates.

### 7. Message Delivery Flow
Taking User A sending a private message to User B as an example:
1. A's client connects to some service node S (can be arbitrarily chosen).
2. The client sends a request to S: obtain B's device public key list in the target subnetwork. S obtains and returns this from the subnetwork's maintenance nodes (requires comparison from multiple nodes, or trust a public key bundle signed by B).
3. A performs an X3DH handshake using B's public key, generates a session key, and encrypts the message content.
4. A sends the encrypted message bundle (containing recipient identifiers: subnet_id, B's user_id) to S.
   - **Subnetwork Isolation Principle**: Messages are only delivered within the same subnetwork and are not routed across subnetworks.
5. S finds the subnetwork's maintenance node set based on subnet_id and forwards the message to these nodes (or selects one node to store offline messages).
6. If B is online, a service node connected to B will receive the message through Gossip or subscription mechanisms and push it to B; if B is offline, maintenance nodes store the message and wait for B to come online to pull it.
7. After B receives the ciphertext, it decrypts using its own device private key and sends an acknowledgment receipt (optional).

Group chat messages are similar, but use group symmetric key encryption, with the recipient being channel_id, and maintenance nodes distributing the message to all online members in the group (or storing as group chat history).

**Cross-subnetwork Forwarding**: If a message needs to be forwarded to another subnetwork, it must be replicated in the target subnetwork rather than directly referenced. After replication, the message in the target subnetwork has an independent lifecycle.

### 8. Security Considerations and Malicious Assumption Mitigation
- **Malicious Service Nodes**:
  - **Data Tampering**: All important data (member lists, public keys) has initiator signatures, clients can verify signatures, and obtain cross-comparison from multiple nodes.
  - **Denial of Service**: Clients can switch to other connected service nodes; if subnetwork maintenance nodes collectively act maliciously, clients can discover this through social channels and rebuild the subnetwork.
  - **Metadata Eavesdropping**: End-to-end encryption protects content, but service nodes can still see communication relationships (who is talking to whom). Mix networks may be introduced later to hide relationships.
- **Malicious Clients**:
  - **Resource Abuse**: Service nodes implement rate limiting for each client, and can require clients to perform difficulty-based proof-of-work (e.g., Hashcash) or pay micro-tokens (if an economic model is introduced).
  - **Identity Spoofing**: All client operations require signing with the identity private key, and service nodes verify signatures to prevent forgery.
  - **Sybil Attack**: Joining a subnetwork requires administrator review, preventing attackers from creating a large number of fake identities; global identity registration may consider combining external anchors (e.g., email verification) but is not mandatory.
- **Subnetwork Maintenance Node Collusion**:
  - If more than half of the maintenance nodes in a subnetwork are malicious, they can tamper with data or deny service. For this reason, subnetwork creators can require maintenance nodes to provide collateral (e.g., locked tokens) when selecting them, or allow clients to obtain data from trusted third parties at the protocol level (e.g., social graph verification). A "witness node" mechanism can be introduced in the design, allowing ordinary nodes to subscribe to subnetwork data and generate consistency proofs, which clients can rely on.

### 9. Client-Server Interaction Overview (Reference)
Client functionality refers to IM products, with main interactions including:
- **Login**: Provide `auth_id` and signed challenge to obtain a session token.
- **Subnetwork Switching**: The client lists subnetworks the user has joined (local cache), and the UI displays the corresponding context after switching.
- **Contacts**: Obtain user lists from subnetwork data, supporting search.
- **Send Message**: End-to-end encrypt and send to a service node, which routes to the target subnetwork maintenance nodes.
- **Receive Message**: Receive pushes from service nodes through long polling or WebSocket.
- **Group Management**: Create groups, invite members, modify group information; operations require signing and submission to subnetwork maintenance nodes.
- **Resource Management**: Upload, download, update, delete resources; resource content hashes are stored in subnetwork data, and content can be stored in distributed storage (e.g., IPFS or similar).
- **File Transfer**: Files are uploaded as resources, and resource references are then sent as messages.

### 10. Protocol Boundaries and Disclaimer

The Skynet protocol clearly defines the following **not involved, not responsible** content:

| Category | Description | Responsible Party |
|----------|-------------|-------------------|
| **Key Management** | Storage, backup, recovery of user identity private keys and device keys | Client |
| **Random Number Generation** | Implementation of CSPRNG (Cryptographically Secure Pseudorandom Number Generator) and entropy sources | Client |
| **Key Derivation** | Algorithms for deriving keys from passwords/passphrases (e.g., PBKDF2, Argon2) | Client |
| **Hardware Security Modules** | Use of HSMs, Secure Elements (SE), Trusted Execution Environments (TEE) | Client |
| **Biometric Authentication** | Fingerprint, facial recognition, and other biometric methods | Client |
| **Post-Quantum Cryptography** | Post-Quantum Cryptography (PQC) migration and key encapsulation mechanisms | Out of scope, not currently involved |
| **Mix Networks** | Metadata hiding technologies such as Loopix | Optional extension, not implemented in initial version |
| **Economic Incentives** | Economic models such as tokens, collateral, proof-of-work | Optional extension |
| **Governance Mechanisms** | Decentralized governance such as DAOs, voting, arbitration | Optional extension |

**Important Notes**:
- The Skynet protocol only defines network communication and data structure specifications
- Client implementers must independently ensure the correct implementation of the above security mechanisms
- Cryptographic security ultimately depends on the correct implementation of the client and the user's security awareness

### 10. Summary
The Skynet protocol provides a foundation for building privacy-preserving communication systems similar to enterprise IM through a decentralized service node network, subnetwork isolation, end-to-end encryption, and Byzantine fault-tolerant data replication. The protocol design fully considers scenarios where the server and client do not trust each other, ensuring that user data ownership and control remain in their own hands.

#### Security Positioning
The security goal of the Skynet protocol is **enterprise-grade practical security**, not military-grade over-design:
- ✅ Protect enterprise communication privacy and data security
- ✅ Resist common network attacks and internal threats
- ✅ Provide good performance and user experience
- ❌ Do not pursue extreme security against nation-state adversaries
- ❌ Do not sacrifice availability for theoretical absolute security

#### Future Expansion Directions
The protocol design reserves expansion space for future security upgrades:
1. **Metadata Protection**: Mix network technology can be introduced to hide communication relationships
2. **Post-Quantum Migration**: Support smooth transition to post-quantum cryptographic algorithms
3. **Consensus Enhancement**: Consensus algorithms and security thresholds can be upgraded as needed
4. **Audit Enhancement**: More granular security audit and compliance functions can be extended
