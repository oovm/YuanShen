# Skynet Protocol Threat Model

## Overview

This document uses the STRIDE threat model framework to analyze the main security threats facing the Skynet protocol and their mitigation measures.

### Design Principles

The Skynet protocol follows these core design principles:

1. **Efficiency First**: Prioritize system performance and user experience while ensuring basic security
2. **Modern Solutions**: Use well-proven modern cryptographic primitives and protocols
3. **Extensible Design**: Architecture supports smooth expansion of future security mechanisms
4. **Practical Security**: Provides enterprise-grade security guarantees without pursuing military-grade over-engineering

### Security Parameter Selection (Efficiency First)

| Parameter | Value | Description |
|-----------|-------|-------------|
| **Signature Algorithm** | Ed25519 | Efficient and secure elliptic curve signature |
| **Key Exchange** | X25519 | Efficient Diffie-Hellman key exchange |
| **Hash Algorithm** | Blake3-256 | High-performance cryptographic hash |
| **Symmetric Encryption** | ChaCha20-Poly1305 | Authenticated encryption, efficient software implementation |
| **Noise Protocol** | Noise_XX_25519_ChaChaPoly_BLAKE2s | Lightweight handshake protocol |
| **X3DH Pre-key Count** | 100 | Balance between security and storage |
| **MLS Cipher Suite** | MLS10_128_HPKEX25519_AES128GCM_SHA256_Ed25519 | Standard MLS suite |

### Extension Capability Reservations

The protocol design reserves the following security extension points:
- Support for mix network (Loopix, etc.) metadata protection
- Support for post-quantum cryptography (PQC) algorithm migration
- Support for pluggable consensus algorithms
- Support for custom permission policy engines

## Threat Model Framework: STRIDE

| Threat Type | Description |
|-------------|-------------|
| **S**poofing | Impersonating another identity |
| **T**ampering | Altering data or code |
| **R**epudiation | Denying having performed an action |
| **I**nformation Disclosure | Disclosing sensitive information |
| **D**enial of Service | Making a system or service unavailable |
| **E**levation of Privilege | Gaining unauthorized privileges |

---

## 1. Spoofing

### 1.1 Node Identity Spoofing
**Threat**: An attacker impersonates a legitimate service node to join the network and trick clients into connecting.

**Mitigation Measures**:
- Each node has an Ed25519 key pair, NodeID = blake3(public_key)
- Inter-node communication uses Noise protocol (XX pattern), mutually verifying NodeID
- Clients can pre-configure a list of trusted seed nodes

### 1.2 User Identity Spoofing
**Threat**: An attacker impersonates another user to send messages or perform actions.

**Mitigation Measures**:
- All user operations require Ed25519 private key signature
- Service nodes only accept operations after verifying signatures
- End-to-end encryption ensures only users with the corresponding private key can decrypt messages

---

## 2. Tampering

### 2.1 Data Tampering
**Threat**: Malicious service nodes alter subnet data (member lists, message history, etc.).

**Mitigation Measures**:
- All important data is signed by the initiator
- Clients obtain data from multiple maintenance nodes and cross-verify
- Immutable operation logs + hash snapshots can detect historical tampering
- Majority consensus principle in case of conflicts

### 2.2 Message Tampering
**Threat**: An attacker alters message content during transmission.

**Mitigation Measures**:
- End-to-end encryption uses ChaCha20-Poly1305, providing authenticated encryption
- Messages include content_hash (Blake3) for integrity verification
- Recipients verify the Poly1305 tag and discard messages if verification fails

---

## 3. Repudiation

### 3.1 Operation Repudiation
**Threat**: A user denies having sent a certain message or performed an action.

**Mitigation Measures**:
- All operations are accompanied by Ed25519 signatures
- Immutable operation logs are permanently recorded (optional)
- Signatures can be verified by third parties

---

## 4. Information Disclosure

### 4.1 Content Disclosure
**Threat**: Service nodes or man-in-the-middle obtain plaintext message content.

**Mitigation Measures**:
- All messages are end-to-end encrypted (X3DH + Double Ratchet for private chats, MLS for group chats)
- Service nodes only store and forward ciphertext
- Keys are managed by clients, the protocol does not involve key storage

### 4.2 Metadata Disclosure
**Threat**: Service nodes can see communication relationships (who talks to whom, when).

**Mitigation Measures**:
- Subnet isolation: Data from different subnets is completely isolated
- **Extensible Reservation**: Protocol architecture supports future introduction of mix networks (like Loopix) to hide metadata (not mandatory in current version)

### 4.3 auth_id Disclosure
**Threat**: auth_id is disclosed within a subnet, associating user identities across subnets.

**Mitigation Measures**:
- auth_id is only used for initial authentication and never stored in subnet data
- Only user_id is used internally within a subnet
- Explicit protocol specification prohibits auth_id propagation within subnets

---

## 5. Denial of Service (DoS)

### 5.1 Service Node DoS
**Threat**: An attacker consumes service node resources, making it unable to process legitimate requests.

**Mitigation Measures**:
- Peer-to-peer network architecture, clients can switch to other nodes
- Rate limiting: Request frequency per client is restricted
- Decentralization: No single point of failure

### 5.2 Subnet DoS
**Threat**: A large number of malicious nodes become subnet maintenance nodes and deny service.

**Mitigation Measures**:
- Closed subnets: Joining requires administrator approval
- Optional for open subnets: Introduce proof-of-work or staking mechanisms
- Clients can read from multiple nodes, using majority consensus

---

## 6. Elevation of Privilege

### 6.1 Regular Member Privilege Escalation
**Threat**: A regular member gains administrator privileges.

**Mitigation Measures**:
- Permission operations require administrator signature
- Multi-node verification: Requires confirmation from a majority of maintenance nodes
- Operation logs record all permission changes

### 6.2 Service Node Overreach
**Threat**: A service node performs unauthorized operations.

**Mitigation Measures**:
- Zero-trust architecture: Do not assume service nodes are trustworthy
- All critical operations require client signatures
- Service nodes only verify signatures and forward messages

---

## Summary

The Skynet protocol addresses various threats through these core mechanisms:
1. **Cryptographic Signatures**: Prevent spoofing, tampering, and repudiation
2. **End-to-End Encryption**: Prevent content disclosure
3. **Multi-Node Verification**: Prevent single-node misbehavior
4. **Zero-Trust Architecture**: Do not trust any participant
5. **Subnet Isolation**: Limit the scope of attack impact

### Security Positioning

The security goal of the Skynet protocol is **enterprise-grade practical security**, not military-grade over-engineering:
- ✅ Protect enterprise communication privacy and data security
- ✅ Resist common network attacks and insider threats
- ✅ Provide good performance and user experience
- ❌ Does not pursue resisting full-scale attacks from state-level adversaries
- ❌ Does not sacrifice usability for theoretical absolute security

### Future Expansion Directions

The protocol design reserves expansion space for future security upgrades:
1. **Metadata Protection**: Can introduce mix network technology to hide communication relationships
2. **Post-Quantum Migration**: Support smooth transition to post-quantum cryptographic algorithms
3. **Consensus Enhancement**: Can upgrade consensus algorithms and security thresholds as needed
4. **Audit Enhancement**: Can extend more fine-grained security audit and compliance features
