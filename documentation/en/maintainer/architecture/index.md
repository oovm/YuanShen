# Architecture Design

This directory contains documentation on AI Company's architectural design philosophy and principles.

## Documentation List

- [decentralization.md](decentralization.md) - Decentralized Design: Core Security Design Principles of the System
- [security-model.md](security-model.md) - Security Model: Architecture-level Security Implementation Guide Based on Skynet Protocol
- [presentation-layer.md](presentation-layer.md) - Presentation Layer: Detailed Documentation of Frontend Applications and Backend Services
- [implementation-layer.md](implementation-layer.md) - Implementation Layer: Detailed Documentation of augur-* Core Modules
- [protocol-layer.md](protocol-layer.md) - Protocol Layer: Detailed Documentation of Skynet Protocol Modules

## Three-Layer Architecture

AI Company employs a clear three-layer architecture design, with each layer having well-defined responsibilities and boundaries.

### Architecture Layer Relationship Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                   Presentation Layer (Application)            │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (Main App)   │  │  (Empire Module)│  │  (Planet Module)│    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │  ai-waifu    │  │  Frontend App │                         │
│  │  (Role Module)│  │  (Vue.js)     │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
                            ↓ Calls
┌─────────────────────────────────────────────────────────────┐
│              Implementation Layer (augur-* Core Modules)     │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │    │
│  │(Agent Management)│  │ator(Orchestrator)│  │ation(Organization)│    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │augur-skill   │  │augur-memory  │  │augur-persist │    │
│  │(Skill Management)│  │(Memory System)│  │ence(Persistence)│    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │augur-file-sys│  │augur-types   │                         │
│  │tem(File System)│  │(Type Definitions)│                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
                            ↓ Uses
┌─────────────────────────────────────────────────────────────┐
│                  Protocol Layer (Skynet Protocol)            │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-types  │  │skynet-auth   │  │skynet-chat   │    │
│  │(Basic Types)  │  │(Auth Protocol)│  │(Chat Protocol)│    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-gateway│  │skynet-memory │  │skynet-service│    │
│  │(Gateway Protocol)│  │(Storage Protocol)│  │(Service Protocol)│    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │skynet-bridge │  │skynet-notific│  │skynet-persist│    │
│  │(Bridge Protocol)│  │ation(Notification)│  │ence(Persistence)│    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities Documentation

#### 1. Presentation Layer (Application)

The presentation layer is the interface with which users directly interact, including various application programs and frontend interfaces.

**Main Components:**
- `ai-company` - Main application backend
- `ai-empire` - Empire module backend
- `ai-planet` - Planet module backend
- `ai-waifu` - Role module backend
- Vue.js frontend application

**Responsibilities:**
- Provide user interface
- Handle user interactions
- Call the implementation layer's core modules
- Data display and collection

#### 2. Implementation Layer (augur-* Core Modules)

The implementation layer is the system's core business logic layer, containing multiple specialized functional modules.

**Main Modules:**
- `augur-agent` - Agent Management
- `augur-orchestrator` - Orchestrator
- `augur-organization` - Organization Management
- `augur-skill` - Skill Management
- `augur-memory` - Memory System
- `augur-persistence` - Persistence
- `augur-file-system` - File System
- `augur-types` - Type Definitions

**Responsibilities:**
- Implement core business logic
- Coordinate module collaboration
- Manage agents and skills
- Provide organization collaboration features

#### 3. Protocol Layer (Skynet Protocol)

The protocol layer provides low-level communication and data interaction protocols, serving as the infrastructure for the entire system.

**Main Protocols:**
- `skynet-types` - Basic Type Definitions
- `skynet-auth` - Authentication Protocol
- `skynet-chat` - Chat Protocol
- `skynet-gateway` - Gateway Protocol
- `skynet-memory` - Storage Protocol
- `skynet-service` - Service Protocol
- `skynet-bridge` - Bridge Protocol
- `skynet-notification` - Notification Protocol
- `skynet-persistence` - Persistence Protocol

**Responsibilities:**
- Provide standardized communication protocols
- Handle identity authentication and security
- Manage data storage and transmission
- Support decentralized communication

## Relationship with Skynet

The Skynet Protocol forms the infrastructure layer (protocol layer) of the AI Company system. This directory describes AI Company's **design philosophy and architectural principles**, while the [Skynet Protocol](../skynet/index.md) is the **specific technical implementation** of these design philosophies.

### Skynet Positioning in the Three-Layer Architecture

In the three-layer architecture, the Skynet Protocol is the lowest-level infrastructure:

```
Presentation Layer (Application)
    ↓ Calls
Implementation Layer (augur-* Core Modules)
    ↓ Uses
Protocol Layer (Skynet Protocol) ← This is where Skynet is located
```

### Overall Relationship Diagram

```
AI Company Core Concepts (concepts/)
    ↓ Built on
Three-Layer Architecture Implementation (Presentation + Implementation Layers)
    ↓ Based on
Skynet Protocol (Protocol Layer)
    ↓ Follows
Decentralized Design Philosophy (architecture/)
```

### AI Company Concepts to Skynet Concepts Mapping

AI Company's core concepts are built on top of the Skynet Protocol:

| AI Company Concept | Skynet Equivalent Concept | Description |
|-------------------|---------------------------|-------------|
| **Organization** | **Subnet** | Each organization corresponds to an independent subnet |
| **User (Real Human User)** | **Subnet User (user_id/auth_id)** | Real users correspond to user identities within a subnet |
| **Address Book - Internal Contacts** | **user_id** | Internal contacts only require user_id |
| **Address Book - External Contacts** | **subnet_id + user_id** | External contacts require the subnet_id + user_id tuple |
| **Organization Structure** | **AI Company Exclusive** | Organization structure is a concept built by AI Company on top of subnets |
| **Team (Agent Cluster)** | **AI Company Exclusive** | Teams are AI Company's exclusive concept for agent collaboration units |
| **Project** | **AI Company Exclusive** | Projects are bound to a channel (cannot change) and a team (can change), with the concept of project administrators |
| **Channel** | **Channel** | Channels are Skynet's native chat group concept, and cannot be changed once bound to a project |
| **Task** | **AI Company Exclusive** | Tasks are work unit concepts under projects |
| **Employee (AI Agent)** | **AI Company Exclusive** | AI Agent employees are container concepts for skill collections |
| **Skill** | **AI Company Exclusive** | Skills are intrinsic work capability units of Agents |
| **Capability** | **AI Company Exclusive** | Capabilities are external extended functions of Agents (including APIs, MCP, etc.) |
| **Workspace** | **AI Company Exclusive** | Workspaces are AI Company's exclusive concept for distributed collaboration |
| **Worker Node** | **Service Node** | Worker nodes are Skynet's service nodes |
| **Workflow** | **AI Company Exclusive** | Workflows are AI Company's exclusive concept for task orchestration |
| **Schedule/ Timed Tasks** | **AI Company Exclusive** | Schedules/timed tasks are AI Company's exclusive concept for time-driven tasks |
| **Work Report** | **AI Company Exclusive** | Work reports (daily, weekly, monthly) are concepts for summarizing the work achievements of AI Agents, teams, and tasks |

### Design Philosophy to Technical Implementation Mapping

| Design Philosophy | Skynet Implementation |
|------------------|---------------------|
| Zero Trust Architecture | Node identity verification, end-to-end encryption |
| Decentralization | Peer-to-peer network, mainnet-subnet two-level structure |
| Verifiable Computing | Digital signatures, multi-node cross-verification |
| Data Sovereignty | End-to-end encryption, user-controlled keys |

See [Skynet Protocol](../skynet/index.md) for specific technical implementations, and see [Core Concepts](../../concepts/index.md) for AI Company's upper-level concepts.
