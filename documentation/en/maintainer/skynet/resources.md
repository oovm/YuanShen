# Resource Model

Resources are entities accessible within a subnet, stored and managed by subnet maintenance nodes.

## Resource Attributes

| Attribute | Description | Type |
|-----------|-------------|------|
| **resource_id** | Unique resource ID | String |
| **resource_type** | Resource type | String (e.g., file/image/document/link, etc., open type) |
| **name** | Resource name | String |
| **description** | Resource description | String (optional) |
| **content_hash** | Resource content hash | blake3 hash (for integrity verification) |
| **content_uri** | Resource content storage location | URI (optional, e.g., IPFS CID, local storage, etc.) |
| **size** | Resource size | Integer (bytes) |
| **created_at** | Creation time | Timestamp |
| **created_by** | Creator | user_id |
| **updated_at** | Update time | Timestamp (optional) |
| **updated_by** | Updater | user_id (optional) |
| **version** | Version number | Integer |
| **permissions** | Permission settings | JSON object (optional) |
| **metadata** | Custom metadata | JSON object (optional) |

## Resource Types

Resource types are open string identifiers, common examples include:

| Resource Type | Description |
|---------------|-------------|
| **file** | General file |
| **image** | Image file |
| **document** | Document file |
| **video** | Video file |
| **audio** | Audio file |
| **link** | Link resource |

## Resource Operations

- **Resource Upload**: Create a new resource, calculate content_hash, store content
- **Resource Download**: Retrieve resource by resource_id, verify content_hash
- **Resource Update**: Update resource content, increment version number, retain historical versions
- **Resource Delete**: Mark resource as deleted (soft delete)
- **Resource Query**: Search and filter resource list

## Resource Permissions

- Resource creators can set access permissions
- Supports permission levels such as read-only, read-write, and management
- Can be authorized to specific users, roles, or entire channels

## Subnet Isolation Principle

- Resources **do not cross subnets** for sharing; resources are only accessible within their associated subnet
- If a resource needs to be used across subnets, it must be **copied** to the target subnet instead of being directly referenced
- After copying, the replica in the target subnet has an independent lifecycle and permissions