# Implementation Layer (augur-* Core Modules)

This document describes the core modules of the implementation layer in the AI Company system. The implementation layer sits between the presentation layer and protocol layer, providing core business logic and functional implementations.

## Overview

The implementation layer consists of multiple independent `augur-*` modules, each responsible for a specific functional area. These modules collaborate through dependency injection and clear interface design, providing a solid functional foundation for upper-layer applications.

## Module List

The implementation layer includes the following core modules:

| Module Name | Responsibility | Status |
|-------------|----------------|--------|
| [augur-agent](#augur-agent) | Agent Management | Core |
| [augur-orchestrator](#augur-orchestrator) | Task Orchestration and Workflow Management | Core |
| [augur-organization](#augur-organization) | Organization, Department, and Role Management | Core |
| [augur-skill](#augur-skill) | Skill Plugins and Capability Extensions | Core |
| [augur-memory](#augur-memory) | Memory System Management | Core |
| [augur-persistence](#augur-persistence) | Persistence Layer Implementation | Infrastructure |
| [augur-file-system](#augur-file-system) | File Storage Service | Infrastructure |
| [augur-types](#augur-types) | Type Definitions and Error Handling | Infrastructure |

## Module Dependency Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                      Business Logic Layer                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-agent   │  │augur-orchestr│  │augur-organiz │          │
│  │(Agent)       │  │ator          │  │ation          │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-skill   │  │augur-memory  │  │              │          │
│  │(Skill)        │  │(Memory)       │  │              │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ Depends on
┌─────────────────────────────────────────────────────────────────┐
│                    Infrastructure Layer                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │augur-persist │  │augur-file-sys│  │augur-types   │          │
│  │ence          │  │tem            │  │               │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
                            ↓ Uses
┌─────────────────────────────────────────────────────────────────┐
│                   Protocol Layer (Skynet)                        │
└─────────────────────────────────────────────────────────────────┘
```

## Module Details

### augur-types

**Responsibility:** Provides shared type definitions and unified error handling mechanisms for the implementation layer.

**Key Features:**
- Defines core data structures (workspace, project, company, team, employee, skill, etc.)
- Unified error type `AugurError` and error handling mechanism
- Provides standard enumeration types (such as `ProjectStatus`, `RoleType`, `SkillLevel`, etc.)
- Supports internationalized error messages (via i18n keys)

**Core Structures:**
- `Workspace` - Workspace
- `Project` - Project
- `Company` - Company
- `Team` - Team
- `Employee` - Employee
- `Skill` - Skill

**Dependencies:** No other augur-* module dependencies; serves as the foundation for all other modules.

### augur-persistence

**Responsibility:** Provides a unified persistence layer implementation supporting multiple database backends.

**Key Features:**
- Multi-database support (SQLite, PostgreSQL)
- Unified Repository interface design
- Database migration management
- Entity type persistence support

**Supported Entity Types:**
- User
- Organization
- Department
- Role
- Conversation
- Message
- Agent
- Memory
- Memory Tag

**Usage Example:**
```rust
use augur_persistence::{AugurPersistence, PersistenceConfig, DatabaseType};

let config = PersistenceConfig::default();
let persistence = AugurPersistence::new(config).await?;

let user_repo = persistence.user_repository();
```

**Dependencies:** Depends on `augur-types`; depended on by business logic modules.

### augur-file-system

**Responsibility:** Provides file storage services based on the local file system.

**Key Features:**
- File upload and download
- File metadata management
- File access level control
- File search and filtering
- User and organization file listing
- File copy and move operations
- Multiple MIME type detection support

**Usage Example:**
```rust
use augur_file_system::{FsFileService, FsFileServiceConfig};

let config = FsFileServiceConfig::default();
let service = FsFileService::new(config).await?;
```

**Dependencies:** Depends on `augur-types`; depended on by business logic modules.

### augur-memory

**Responsibility:** Implements the memory system, supporting memory creation, storage, retrieval, and management.

**Key Features:**
- Memory CRUD operations
- Memory tag management
- Memory relationship management
- Keyword search
- Tag and time range filtering
- Memory import and export
- Conversation context extraction
- Memory creation from conversations

**Usage Example:**
```rust
use augur_memory::AugurMemory;
use std::sync::Arc;

let memory_service = AugurMemory::new(
    Arc::new(memory_repository),
    Arc::new(memory_tag_repository),
);
```

**Dependencies:** Depends on `augur-types`, `augur-persistence`; depended on by `augur-agent`, etc.

### augur-agent

**Responsibility:** Provides type definitions and interfaces related to Agents, managing Agent lifecycles.

**Key Features:**
- Agent type definitions
- Agent state management
- Agent configuration interfaces
- Agent lifecycle management

**Dependencies:** Depends on `augur-types`, `augur-memory`, `augur-skill`; depended on by upper-layer applications.

### augur-orchestrator

**Responsibility:** Provides types and interfaces for task orchestration and workflow management.

**Key Features:**
- Task orchestration interfaces
- Workflow management
- Task scheduling
- Dependency management

**Dependencies:** Depends on `augur-types`; depended on by upper-layer applications.

### augur-organization

**Responsibility:** Provides types and interfaces for organization, department, and role management.

**Key Features:**
- Organization management
- Department structure
- Role permissions
- Member management

**Dependencies:** Depends on `augur-types`, `augur-persistence`; depended on by upper-layer applications.

### augur-skill

**Responsibility:** Provides types and interfaces for skill plugins and capability extensions.

**Key Features:**
- Skill plugin interfaces
- Skill definitions
- Skill execution
- Plugin management

**Dependencies:** Depends on `augur-types`; depended on by `augur-agent`.

## Module Collaboration Examples

### Example 1: Agent Task Execution

1. Upper-layer application calls `augur-orchestrator` to initiate a task
2. `augur-orchestrator` finds available `augur-agent`
3. `augur-agent` uses `augur-skill` to load required skills
4. During execution, `augur-agent` uses `augur-memory` to record and retrieve memories
5. All state changes are persisted via `augur-persistence`

### Example 2: Organization Collaboration

1. Upper-layer application manages organization structure through `augur-organization`
2. `augur-organization` uses `augur-persistence` to store organization data
3. User uploaded files are managed via `augur-file-system`
4. Agents within the organization collaborate through `augur-agent`

## Design Principles

The implementation layer follows these design principles:

1. **Single Responsibility Principle** - Each module is responsible for only one clear functional area
2. **Dependency Inversion Principle** - High-level modules do not depend on low-level modules; both depend on abstractions
3. **Interface Segregation Principle** - Provides minimal interfaces, avoiding fat interfaces
4. **Modular Design** - Modules collaborate through clear boundaries and dependency relationships
5. **Testability** - Each module should be easy to test independently

## Extension Guide

When adding new functionality, follow these steps:

1. Determine which existing module the functionality belongs to, or whether a new module needs to be created
2. Define necessary data structures and error types in `augur-types`
3. Implement core logic in the corresponding business module
4. If persistence is required, add a Repository in `augur-persistence`
5. If file storage is required, use `augur-file-system`
6. Update this document to record descriptions of the new functionality

## Related Documents

- [Architecture Overview](./index.md)
- [Decentralization](./decentralization.md)
- [Security Model](./security-model.md)
- [Data Models](../data-models.md)
- [Skynet Protocol](../skynet/index.md)
