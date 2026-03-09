# Workspaces

## Core Concepts

### Workspace

A **workspace** is a logical unit in the AI Company system used to isolate and organize different tasks, projects, or environments. **Workspaces can exist and run across multiple worker nodes**, enabling true distributed collaboration.

Workspaces are no longer limited to a single node, but rather a logical entity that can span across device boundaries.

#### Workspace Roles

1. **Environment Isolation**
   - Data and configurations from different workspaces are isolated from each other
   - Prevents interference between tasks
   - Supports independent security policies

2. **Cross-Node Collaboration**
   - Workspaces can run on multiple nodes simultaneously
   - Agents can migrate and collaborate across different nodes
   - Data automatically synchronizes across multiple nodes

3. **Resource Allocation**
   - Allocates specific resources to each workspace (can span across nodes)
   - Limits resource usage to prevent resource exhaustion
   - Priority-based scheduling

4. **Context Management**
   - Saves the context state of the workspace
   - Quickly switches between different work scenarios
   - Supports workspace pause and resume

#### Workspace Types

| Workspace Type | Purpose | Characteristics |
|---------------|---------|-----------------|
| **Personal Workspace** | Personal daily use | Fully private, flexible configuration |
| **Project Workspace** | Specific project collaboration | Shareable, version control |
| **Agent Workspace** | AI agent execution | Automatic management, resource optimization |
| **Secure Workspace** | Sensitive task processing | Highest security level, audit tracking |
| **Temporary Workspace** | Short-term tasks | Automatic cleanup, short lifecycle |

#### Workspace-Node Relationship

```
Workspace (cross-node)
│
├── Worker Node 1 (Windows PC)
│   ├── Workspace Instance A
│   │   ├── Agent 1
│   │   └── Local data cache
│   └── Workspace Instance B
│       └── ...
│
├── Worker Node 2 (Android Phone)
│   └── Workspace Instance A
│       ├── Agent 2 (migrated from Node 1)
│       └── Local data cache
│
└── Worker Node 3 (Server)
    └── Workspace Instance A
        ├── Agent 3 (running in background)
        └── Complete data copy
```

## Workspace Management

### Workspace Creation

1. **Creation Methods**
   - Manual creation: User actively creates a workspace
   - Automatic creation: Automatically created based on task requirements
   - Template creation: Created based on preset templates

2. **Workspace Configuration**
   ```
   Workspace Configuration {
     name: Workspace name
     type: personal | project | agent | secure | temp
     nodeIds: [Associated node list]  // Can span across multiple nodes
     resources: {
       cpu: CPU limit (total across nodes)
       memory: Memory limit (total across nodes)
       storage: Storage limit (total across nodes)
     }
     security: {
       encryption: Whether to encrypt
       accessControl: Access control
     }
     lifecycle: {
       autoCleanup: Whether to auto-cleanup
       ttl: Time to live
     }
     distribution: {
       preferredNodes: [Preferred nodes]
       failoverNodes: [Failover nodes]
       replicationStrategy: full | partial | selective
     }
   }
   ```

### Workspace Distribution Strategies

1. **Master-Slave Mode**
   - One master node responsible for coordination
   - Other nodes act as replicas
   - Suitable for read-write separation scenarios

2. **Peer-to-Peer Mode**
   - All nodes have equal status
   - Data replicated between nodes
   - Suitable for high availability scenarios

3. **Sharding Mode**
   - Workspace data sharded across different nodes
   - Each node responsible for a portion of data
   - Suitable for large-scale data scenarios

### Workspace Switching

1. **Quick Switch**
   - One-click workspace switching
   - Saves current context
   - Restores target workspace state

2. **Parallel Workspaces**
   - Supports multiple workspaces running simultaneously
   - Resource isolation between workspaces
   - Configurable workspace priorities

### Workspace Synchronization

1. **Cross-Node Synchronization**
   - Workspace data automatically synchronized across multiple nodes
   - Incremental synchronization to reduce bandwidth
   - Conflict resolution strategies

2. **Selective Synchronization**
   - Can select which nodes to synchronize with
   - Configurable synchronization direction
   - Supports offline editing

3. **Agent Migration**
   - Agents can migrate between different nodes of a workspace
   - Maintains agent state and context
   - Automatic scheduling based on resource conditions

## Node Instance Management

Each workspace has an instance on its associated nodes:

| Instance Status | Description |
|-----------------|-------------|
| **Active** | Instance is active, running agents |
| **Standby** | Instance is on standby, data synchronizing |
| **Offline** | Node is offline, data pending synchronization |
| **Failed** | Instance is abnormal, requires recovery |

## Security Model

### Workspace Security

- **Workspace Encryption**: Workspace data stored encrypted
- **Access Control**: Fine-grained access permission control
- **Audit Logs**: Complete operation audit records
- **Node Authorization**: Controls which nodes can join the workspace

## Relationship with Workflows

Workspaces provide the execution environment for workflows, and workflows execute within workspaces. Workflows can execute across multiple node instances of a workspace, enabling distributed task flow.

See the [Workflows](./workflows.md) document for more information.
