# Worker Nodes

## Core Concepts

### Worker Node

A **Worker Node** is the fundamental computing unit in the AI Company system. **Every physical or virtual device is an independent worker node**.

Worker nodes provide the runtime environment for workspaces. A single workspace can run across multiple worker nodes simultaneously, with one instance of the workspace on each node.

#### Worker Node Types

The system supports the following types of worker nodes:

| Node Type | Description | Typical Scenarios | Characteristics |
|-----------|-------------|------------------|-----------------|
| **Windows Personal PC** | Desktop device | Daily office work, content creation | High performance, large screen, complete input devices |
| **Server** | Cloud or on-premises server | Background tasks, long-running services | 24/7 operation, high reliability, multi-task processing |
| **Android Phone** | Android mobile device | Mobile office, immediate response | Portability, anytime/anywhere access, rich sensors |
| **iOS Phone** | Apple mobile device | Mobile office, immediate response | Portability, high security, mature ecosystem |
| **Tablet Device** | Android/iPad tablet | Content consumption, light creation | Large screen, touch-friendly |
| **IoT Device** | Smart hardware | Specific tasks, data collection | Dedicated functionality, low power consumption |

#### Worker Node Characteristics

Each worker node possesses the following characteristics:

1. **Independent Identity**
   - Each node has a unique node ID
   - Supports node authentication and authorization
   - Can independently join or leave the system

2. **Local-First**
   - Prioritizes local computing resources
   - Continues to function normally while offline
   - Data synchronization occurs automatically in the background

3. **Resource Discovery**
   - Automatic node discovery and connection
   - Dynamic adjustment of node topology
   - Supports hot-plugging of nodes

4. **Capability Declaration**
   - Nodes declare their capabilities (e.g., GPU, specific software, sensors)
   - Tasks are routed to appropriate nodes
   - Capabilities can be dynamically updated

## Architecture Design

### Node-Workspace Hierarchy

```
AI Company System
│
├── Workspace A (Personal Workspace, across 3 nodes)
│   ├── Worker Node 1 (Windows PC)
│   │   └── Workspace Instance A-1
│   │       ├── Agent 1
│   │       └── Local Data Cache
│   ├── Worker Node 2 (Android Phone)
│   │   └── Workspace Instance A-2
│   │       ├── Agent 2 (migrated from Node 1)
│   │       └── Local Data Cache
│   └── Worker Node 3 (Server)
│       └── Workspace Instance A-3
│           ├── Agent 3 (running in background)
│           └── Complete Data Copy
│
├── Workspace B (Project Workspace, across 2 nodes)
│   ├── Worker Node 1 (Windows PC)
│   │   └── Workspace Instance B-1
│   │       └── Agent 4
│   └── Worker Node 4 (Server)
│       └── Workspace Instance B-2
│           └── Agent 5 (long-running)
│
└── Workspace C (Temporary Workspace, single node)
    └── Worker Node 2 (Android Phone)
        └── Workspace Instance C-1
            └── Temporary Task
```

### Inter-Node Communication

Worker nodes support automatic discovery, secure communication, and data synchronization. For detailed technology selection, refer to [Technology Choices](../maintainer/technology-choices.md) in the maintainer documentation.

## Worker Node Management

### Node Registration

1. **First Startup**
   - Node generates a unique identity key pair
   - Creates node configuration file
   - Registers with the system

2. **Node Information**
   ```
   Node Info {
     id: Unique node identifier
     name: Node name
     type: Windows | Server | Android | iOS
     capabilities: [Capability list]
     status: online | offline | busy
     lastSeen: Last online time
     workspaces: [Associated workspace list]
   }
   ```

### Node Status

Worker nodes have multiple states indicating availability and load. For details, refer to the maintainer documentation.

### Node Monitoring

- **Resource Monitoring**: CPU, memory, disk, network
- **Health Checks**: Periodic heartbeat detection
- **Performance Metrics**: Task completion rate, response time
- **Alert Mechanism**: Notifications for abnormal conditions

## Workspace Instance Management

Each workspace has one instance on each associated node:

| Instance Status | Description |
|-----------------|-------------|
| **Active** | Instance is active, running agents |
| **Standby** | Instance is on standby, data synchronizing |
| **Offline** | Node is offline, data pending synchronization |
| **Failed** | Instance is abnormal, requires recovery |

### Instance Lifecycle

1. **Create**: Instance created when workspace is assigned to node
2. **Start**: Instance initializes, loads data
3. **Run**: Executes agents, processes tasks
4. **Sync**: Synchronizes data with other instances
5. **Stop**: Pauses agents, saves state
6. **Destroy**: Removes instance from node

## Agent Scheduling

### Node Selection

The system selects appropriate worker nodes (within the workspace scope) based on:

1. **Capability Matching**: Whether the node has required capabilities
2. **Resource Availability**: Whether node resources are sufficient
3. **Network Latency**: Prioritizes nodes with lower latency
4. **Data Location**: Prioritizes nodes where data resides
5. **User Preference**: Respects user priority choices
6. **Instance Status**: Prioritizes instances in Active state

### Task Migration

- **Dynamic Migration**: Agents can migrate between different nodes within a workspace
- **State Preservation**: Agent state and context preserved during migration
- **Seamless Switching**: Users do not perceive the migration process

## Data Management

### Data Storage Hierarchy

1. **Instance Level**: Local cache of workspace instance
2. **Workspace Level**: Shared data for workspace (synchronized across nodes)
3. **Global Level**: Shared data across workspaces

### Data Synchronization Strategies

- **Real-time Sync**: Critical data synchronized in real-time
- **Periodic Sync**: Non-critical data synchronized periodically
- **On-demand Sync**: User manually triggers synchronization
- **Conflict Resolution**: Automatic or manual conflict resolution

## Security Model

### Node Security

- **Node Authentication**: Each node has an independent identity certificate
- **Secure Boot**: Security validation performed when node starts
- **Remote Wipe**: Data on lost devices can be remotely erased
- **Workspace Authorization**: Controls which workspaces a node can join

## Relationship with Workflows

Worker nodes provide computing resources for workflows. Workflow tasks can be dispatched to different nodes for execution. Automatic selection based on node capabilities and resources, supporting task migration between nodes.

See [Workflows](./workflows.md) documentation for more details.

## Summary

The concepts of worker nodes and workspaces provide AI Company with a flexible, efficient, and secure distributed computing architecture:

- **Flexibility**: Workspaces can span nodes, supporting various device types and adapting to different scenarios
- **Efficiency**: Intelligent scheduling, optimized resource utilization, automatic agent migration
- **Security**: Multi-layer isolation, fine-grained permission control, node authorization
- **User Experience**: Seamless switching, offline availability, data synchronization

This design allows users to truly own and control their computing resources while enjoying the benefits of distributed systems. Workspaces are no longer limited to a single device but are logical entities that can cross node boundaries.
