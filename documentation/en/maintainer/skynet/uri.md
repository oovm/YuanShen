# Skynet URI Protocol

Skynet Protocol uses `skynet://` as the Uniform Resource Identifier (URI) protocol for referencing various resources in the Skynet network.

## Protocol Format

### Basic Structure

```
skynet://[subnet_id]/[resource_type]/[resource_id][?query][#fragment]
```

### Component Description

| Component | Description | Required |
|-----------|-------------|----------|
| **skynet://** | Protocol header | Yes |
| **subnet_id** | Subnet ID | Yes |
| **resource_type** | Resource type | No |
| **resource_id** | Resource ID | No |
| **query** | Query parameters | No |
| **fragment** | Fragment identifier | No |

## Resource Types

### Supported Resource Types

| Resource Type | Description | Path Format |
|---------------|-------------|-------------|
| **subnet** | Subnet | `skynet://{subnet_id}` |
| **user** | User | `skynet://{subnet_id}/user/{user_id}` |
| **channel** | Channel | `skynet://{subnet_id}/channel/{channel_id}` |
| **message** | Message | `skynet://{subnet_id}/message/{message_id}` |
| **resource** | Resource (files, etc.) | `skynet://{subnet_id}/resource/{resource_id}` |
| **thread** | Message thread | `skynet://{subnet_id}/thread/{thread_id}` |

## Examples

### 1. Reference a Subnet

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99
```

### 2. Reference a User

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/user/user_123
```

### 3. Reference a Channel

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/channel/channel_456
```

### 4. Reference a Message

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789
```

### 5. Reference a Resource

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/resource/res_abc
```

### 6. Reference a Thread

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/thread/msg_789
```

### 7. With Query Parameters

```
skynet://5f4dcc3b5aa765d61d8327deb882cf99/message/msg_789?highlight=true&scroll_to_bottom=true
```

## Encoding Specifications

- **subnet_id**: Use hexadecimal representation of Blake3 hash (lowercase)
- **user_id**, **channel_id**, **message_id**, **resource_id**, **thread_id**: Use URL-safe Base64 encoding or hexadecimal
- **Special characters**: Special characters in path components must be URL-encoded

## Subnet Isolation Principle

Skynet URIs follow the subnet isolation principle:
- All URIs must include `subnet_id`
- Resources are not shared across subnets; replication is required for cross-subnet use
- Resources in a URI are only accessible within the specified subnet

## Client Handling

When parsing `skynet://` URIs, clients should:
1. Validate URI format
2. Check if the target subnet has been joined
3. Perform corresponding operations based on resource type (e.g., navigate, open resource, etc.)
4. Provide join guidance for subnets that haven't been joined
