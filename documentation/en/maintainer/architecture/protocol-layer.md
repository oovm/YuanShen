# Protocol Layer Design

This document provides a detailed explanation of the protocol layer of the AI Company system, specifically the Skynet protocol modules in the protocols/ directory. The protocol layer is the infrastructure of the entire system, providing standardized communication protocols, data type definitions, and interaction interfaces.

## Protocol Layer Overview

The protocol layer is the lowest layer in AI Company's three-tier architecture, providing foundational services and standardized interaction protocols for the upper implementation layer (augur-* core modules) and presentation layer.

### Role of the Protocol Layer in the Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Presentation Layer (Applications)            │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
                            ↓ Calls
┌─────────────────────────────────────────────────────────────┐
│              Implementation Layer (augur-* Core Modules)       │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
                            ↓ Uses
┌─────────────────────────────────────────────────────────────┐
│              ← Protocol Layer (Skynet Protocol) →            │
├─────────────────────────────────────────────────────────────┤
│  Base Types  Auth Protocol  Chat Protocol  Gateway Protocol  Memory Protocol  Service Protocol  │
│  Bridge Protocol  Notification Protocol  Persistence Protocol  Storage Protocol                │
└─────────────────────────────────────────────────────────────┘
```

### Core Responsibilities

The core responsibilities of the protocol layer include:

1. **Providing standardized data type definitions**: Unifying data exchange formats between modules
2. **Defining communication protocols**: Standardizing interaction methods between modules and services
3. **Providing foundational security mechanisms**: Identity authentication, encrypted transmission, and access control
4. **Supporting decentralized communication**: Mainnet-subnet two-tier structure, peer-to-peer network communication
5. **Abstracting underlying differences**: Providing unified interfaces to upper layers, shielding implementation details

## Protocol Module Details

### 1. skynet-types - Base Type Definitions

**Responsibilities and Features**:
- Provides all shared type definitions for the Skynet protocol layer
- Defines core data structures, enums, and constants
- Provides identity generation and handling mechanisms
- Defines error types and unified error handling specifications
- Provides JSON-RPC and WebSocket message types

**Main Submodules**:
- `agent` - Agent-related types
- `bridge` - Bridge-related types
- `chat` - Chat-related types
- `error` - Error-related types
- `id` - Identity-related types
- `jsonrpc` - JSON-RPC-related types
- `memory` - Memory-related types
- `org` - Organization-related types
- `resource` - Resource-related types
- `subnet` - Subnet-related types
- `user` - User-related types
- `utils` - Utility functions
- `websocket` - WebSocket-related types

**Role in Architecture**: Serves as the foundation for all other protocol modules, providing a unified type system to ensure data compatibility between modules.

---

### 2. skynet-auth - Authentication Protocol

**Responsibilities and Features**:
- Provides user identity authentication interfaces
- Manages permissions and access control
- Handles session management and tokens
- Supports multiple authentication methods

**Core Features**:
- User authentication flow definition
- Permission management and authorization mechanisms
- Session token generation and validation
- Token lifecycle management

**Role in Architecture**: Provides secure identity authentication and permission control foundation for the entire system, ensuring only authorized users and services can access system resources.

---

### 3. skynet-chat - Chat Protocol

**Responsibilities and Features**:
- Provides real-time chat and session management
- Defines message types and formats
- Supports chat history
- Provides real-time communication interfaces

**Core Features**:
- Session (channel) management
- Multiple message type definitions (text, files, images, etc.)
- Real-time message push
- Chat history query and retrieval

**Role in Architecture**: Provides foundational communication support for AI Company's collaboration features, including project channels, team communication, etc.

---

### 4. skynet-gateway - Gateway Protocol

**Responsibilities and Features**:
- Provides API gateway interfaces
- Handles request routing and forwarding
- Implements load balancing
- Performs security verification

**Core Features**:
- Request routing and distribution
- Service discovery and load balancing
- Request validation and security filtering
- API version management

**Role in Architecture**: Serves as the entry point of the system, uniformly managing external requests and providing secure, efficient API access channels.

---

### 5. skynet-memory - Memory Protocol

**Responsibilities and Features**:
- Provides memory storage and retrieval interfaces
- Defines memory data types
- Supports memory search and query
- Manages memory tags and classification

**Core Features**:
- Memory data storage and reading
- Memory vector search
- Tag management and classification
- Memory association and context

**Role in Architecture**: Provides memory system support for agents, enabling agents to remember historical interactions, learn from experience, and form long-term memory.

---

### 6. skynet-service - Service Protocol

**Responsibilities and Features**:
- Provides generic service interface definitions
- Supports service discovery mechanisms
- Implements health checks and monitoring
- Manages service registration and deregistration

**Core Features**:
- Unified service interface specifications
- Service registration and discovery
- Health status monitoring
- Service lifecycle management

**Role in Architecture**: Provides unified registration, discovery, and communication mechanisms for various services in the system, supporting microservice architecture.

---

### 7. skynet-bridge - Bridge Protocol

**Responsibilities and Features**:
- Provides external system integration interfaces
- Handles message forwarding and protocol conversion
- Supports multi-platform integration
- Manages cross-system communication

**Core Features**:
- External system bridging
- Protocol conversion and adaptation
- Message routing and forwarding
- Multi-platform connectors

**Role in Architecture**: Enables AI Company to integrate and communicate with external systems such as third-party services, legacy systems, etc.

---

### 8. skynet-notification - Notification Protocol

**Responsibilities and Features**:
- Provides message notification and push services
- Defines notification types and formats
- Supports multiple push channels
- Manages notification history

**Core Features**:
- Notification service interfaces
- Multi-channel push (mobile, desktop, email, etc.)
- Notification type definitions
- Notification history and status management

**Role in Architecture**: Provides real-time notification capabilities for the system, ensuring users receive important information and updates in a timely manner.

---

### 9. skynet-persistence - Persistence Protocol

**Responsibilities and Features**:
- Provides data repository and persistence interfaces
- Defines entity types and repository patterns
- Supports data query and transaction management
- Provides unified data access layer

**Core Features**:
- Repository interface definitions
- Entity type specifications
- Data query and filtering
- Transaction management and consistency guarantees

**Role in Architecture**: Provides unified data persistence abstraction for the system, shielding underlying database differences and supporting multiple storage backends.

---

### 10. skynet-storage - Storage Protocol

**Responsibilities and Features**:
- Provides generic storage service interfaces
- Supports multiple storage types (files, blobs, objects, etc.)
- Manages storage permissions and access control
- Defines storage operation specifications

**Core Features**:
- Generic storage service interfaces
- Multi-type storage support
- Permission control and access management
- Storage operation definitions (upload, download, delete, etc.)

**Role in Architecture**: Provides unified file and object storage abstraction for the system, supporting various storage needs.

## Protocol Layer Design Principles

### 1. Lightweight and Flexible

The protocol layer only defines the **minimum necessary** interfaces and types, maintaining lightweight and flexibility. Specific implementation details are left to upper layer modules to implement independently based on requirements.

### 2. Standardization and Consistency

All protocol modules follow unified design specifications and naming conventions, ensuring consistency and interoperability between modules.

### 3. Extensibility

Protocol designs reserve extension points to support future feature additions and upgrades without breaking existing interfaces.

### 4. Security First

The protocol layer includes built-in foundational security mechanisms, including identity authentication, encrypted transmission, and access control, ensuring system security.

### 5. Decentralization Support

The protocol layer supports mainnet-subnet two-tier structure and peer-to-peer network communication, providing infrastructure for decentralized applications.

## Interaction with Upper Layers

### Implementation Layer (augur-* Modules)

The core modules of the implementation layer (such as augur-agent, augur-orchestrator, etc.) directly use the interfaces and types provided by the protocol layer:

- `augur-agent` uses `skynet-types` and `skynet-memory`
- `augur-organization` uses `skynet-types` and `skynet-persistence`
- `augur-orchestrator` uses `skynet-service` and `skynet-gateway`

### Presentation Layer (Applications)

The presentation layer uses the protocol layer indirectly through the implementation layer, or in some cases directly uses the base types and interfaces of the protocol layer.

## Summary

The protocol layer is the infrastructure of the AI Company system, providing a solid foundation for the upper implementation layer and presentation layer through standardized communication protocols, data type definitions, and interaction interfaces. Each protocol module has its own responsibilities, together forming a complete, flexible, and secure protocol system that supports the system's decentralized design and enterprise-level application requirements.

Through the abstraction of the protocol layer, AI Company achieves:
- Modular design, with each part able to evolve independently
- Standardized interfaces, facilitating integration and extension
- Security foundation, ensuring system security
- Decentralization support, achieving data sovereignty and user control
