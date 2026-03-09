# Technology Choices

## Overview

This document records the technology selection decisions for the AI Company system, including communication protocols, discovery mechanisms, security standards, and more.

## Inter-Node Communication

### Discovery Mechanism

The system adopts a multi-layer node discovery mechanism:

| Discovery Method | Technology Choice | Purpose |
|------------------|------------------|---------|
| **LAN Discovery** | mDNS/Bonjour | Auto-detect local network nodes |
| **Cloud Discovery** | Cloud Directory Service | Discover remote nodes |
| **Manual Connection** | Direct Address Input | Manually specify node connections |

### Communication Protocols

| Protocol Type | Technology Choice | Purpose |
|---------------|------------------|---------|
| **Real-time Communication** | WebSocket | Bidirectional real-time communication |
| **Service Calls** | gRPC | Efficient inter-service calls |
| **File Synchronization** | rsync or similar protocols | Incremental file synchronization |

### Secure Communication

| Security Mechanism | Technology Choice | Description |
|--------------------|------------------|-------------|
| **Transport Encryption** | TLS 1.3 | All communications encrypted |
| **Node Authentication** | Node Certificates | Mutual authentication |
| **Data Encryption** | End-to-end Encryption | End-to-end encryption for sensitive data |

## Worker Node Technologies

### Supported Node Types

The system supports the following worker node types:
- Windows Personal PC
- Server
- Android Phone
- iOS Phone
- Tablet Device
- IoT Device

### Node Status

| Status | Description |
|--------|-------------|
| **Online** | Node is online and idle |
| **Busy** | Node is online but busy |
| **Offline** | Node is offline |
| **Maintenance** | Node is under maintenance |

## Storage Technology Choices

Refer to [data-models.md](./data-models.md) for detailed storage architecture and technology choices.

### Database Selection

| Database | Purpose | Scenario |
|----------|---------|----------|
| **SQLite** | Lightweight relational database | Single-machine deployment |
| **PostgreSQL** | Enterprise-grade relational database | Production environment |

### Object Storage Selection

| Storage Type | Purpose | Scenario |
|--------------|---------|----------|
| **File System (FS)** | Local file storage | Single-machine deployment |
| **S3** | Object storage service | Production environment and cloud deployment |

## Technology Selection Principles

1. **Open Source First**: Prioritize mature open-source technologies
2. **Standardization**: Follow industry standards and best practices
3. **Scalability**: Choose technologies that support horizontal scaling
4. **Security**: Prioritize security and privacy protection
5. **Performance**: Ensure technology choices meet performance requirements
