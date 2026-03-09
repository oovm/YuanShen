# Message Model

Messages are content units passed within a subnet for communication between users.

## Message Properties

| Property | Description | Type |
|----------|-------------|------|
| **message_id** | Unique message ID | String |
| **channel_id** | Channel ID | Channel ID (group chat) or empty (private chat) |
| **sender_id** | Sender ID | user_id |
| **recipient_id** | Recipient ID | user_id (used in private chats) |
| **message_type** | Message type | String (e.g., text/image/file/resource, open type) |
| **content** | Message content | Structured data (JSON object or encrypted structured data) |
| **content_encrypted** | Whether encrypted | Boolean |
| **resource_refs** | List of resource references | Array of resource_id (optional) |
| **reply_to** | ID of the message being replied to | message_id (optional) |
| **thread_id** | Thread ID | message_id (optional, used for message threads) |
| **mentions** | List of mentions | Array of mention objects (optional) |
| **is_pinned** | Whether pinned | Boolean (optional, default false) |
| **pinned_at** | Pinned timestamp | Timestamp (optional) |
| **pinned_by** | Pinner ID | user_id (optional) |
| **status** | Message status | String (optional, e.g., sending/sent/delivered/read/recalled) |
| **delivered_at** | Delivered timestamp | Timestamp (optional) |
| **read_at** | Read timestamp | Timestamp (optional) |
| **created_at** | Creation timestamp | Timestamp |
| **edited_at** | Edited timestamp | Timestamp (optional) |
| **deleted_at** | Deleted timestamp | Timestamp (optional, soft delete) |
| **metadata** | Custom metadata | JSON object (optional) |

## Message Types

Message types are open string identifiers. Common examples:

| Message Type | Description |
|--------------|-------------|
| **text** | Text message |
| **image** | Image message |
| **file** | File message |
| **resource** | Resource reference message |
| **link** | Link message |
| **system** | System message |

## Message Content Format

- Message content is **structured data** (usually a JSON object)
- Not a fixed Markdown or HTML format
- Client decides how to render based on `message_type` and `content` structure
- Text content may include Markdown, HTML, and other formatting tags, but parsing and rendering is at the client's discretion

### Message Type Content Structure Examples

- **text**: Text message
  - content includes a `text` field (plain text, Markdown, HTML, etc., client decides rendering)
- **image**: Image message
  - content includes a `resource_id` field referencing the image resource
  - May optionally include a `caption` field (image description)
- **file**: File message
  - content includes a `resource_id` field referencing the file resource
  - May optionally include metadata like `filename`, `size`, etc.
- **resource**: Resource reference message
  - content includes a `resource_id` field
- **link**: Link message
  - content includes a `url` field
  - May optionally include preview information like `title`, `description`, `preview_image`, etc.
- **system**: System message (e.g., member joined, channel created, etc.)
  - content includes fields like `event_type`, `event_data`, etc.

## Message Operations

- **Send Message**: Send message to channel or private chat, end-to-end encryption (optional)
- **Receive Message**: Get new messages from maintainer node
- **Edit Message**: Edit sent message, retain edit history
- **Delete Message**: Mark message as deleted (soft delete)
- **Reply to Message**: Reply to a specific message
- **Message History**: Get historical messages from channel or private chat

## Message Security

- Supports end-to-end encryption, only communicating parties can decrypt
- Message content hash for integrity verification
- Sender signature for authentication
- Service nodes cannot access encrypted message content

## Subnet Isolation Principle

- Messages **do not cross subnets**, each subnet is an independent communication domain
- To forward messages across subnets, you must **copy** them to the target subnet instead of directly referencing
- After copying, the copy in the target subnet has an independent lifecycle

## Message Status Management (Optional Extension)

Subnets may choose to enable message status tracking for a richer messaging experience:

| Status | Description |
|--------|-------------|
| **sending** | Message submitted to maintainer node, being processed |
| **sent** | Message stored in maintainer node, waiting to be received |
| **delivered** | Message pushed to recipient's online devices |
| **read** | Recipient has read the message |
| **recalled** | Sender has recalled the message |

### Message Status Operations (Optional)

- **Status Update**: Auto-update to "delivered" when recipient device is online
- **Read Receipt**: Send read notification after recipient reads message (optional)
- **Message Recall**: Sender can recall message within a certain time (optional)
- **Status Visibility**: Configure who can see message status (sender only / both parties / everyone)

## Message Reactions

Message reactions are emotional feedback from users on messages.

### Reaction Properties

| Property | Description | Type |
|----------|-------------|------|
| **reaction_id** | Unique reaction ID | String |
| **message_id** | Target message ID | message_id |
| **user_id** | User who added the reaction | user_id |
| **emoji** | Emoji | String (e.g., "👍", "❤️", "🎉") or custom emoji ID |
| **created_at** | Added timestamp | Timestamp |

### Reaction Operations

- **Add Reaction**: User adds emoji reaction to message
- **Remove Reaction**: User removes their own reaction
- **Query Reactions**: Get all reactions for a message
- **View Users**: View list of users who added a specific emoji

### Reaction Features

- Supports standard Unicode emojis
- Supports custom emojis (needs to be defined at subnet level)
- Same user can only add the same emoji once to the same message
- Same user can add multiple different emojis to the same message

## Message Pins

Message pins are used to mark important messages so they appear at the top of channels or private chats.

### Pin Properties

| Property | Description | Type |
|----------|-------------|------|
| **pin_id** | Unique pin ID | String |
| **message_id** | Pinned message ID | message_id |
| **channel_id** | Channel ID | Channel ID (group chat) or empty (private chat) |
| **pinned_by** | Pinner ID | user_id |
| **pinned_at** | Pinned timestamp | Timestamp |
| **order** | Sort position | Integer (optional, for manual sorting) |

### Pin Operations

- **Pin Message**: Pin message to top of channel or private chat
- **Unpin Message**: Unpin a message
- **Get Pinned List**: Get all pinned messages in a channel or private chat
- **Reorder**: Adjust the display order of pinned messages

### Pin Permissions

- Configure who has permission to pin messages:
  - Administrators only
  - Message sender only
  - All members
  - Custom roles

## Message Threads

Message threads are used for in-depth discussions under specific messages without affecting the main chat flow.

### Thread Properties

| Property | Description | Type |
|----------|-------------|------|
| **thread_id** | Thread ID | message_id (i.e., the parent message's message_id) |
| **parent_message_id** | Parent message ID | message_id |
| **channel_id** | Channel ID | Channel ID (group chat) or empty (private chat) |
| **created_by** | Thread creator | user_id |
| **created_at** | Thread creation timestamp | Timestamp |
| **message_count** | Number of messages in thread | Integer |
| **last_message_at** | Last message timestamp | Timestamp |

### Thread Messages

Messages in a thread are similar to regular messages but with the following differences:
- Thread message's `thread_id` field is set to the parent message's `message_id`
- Thread messages are displayed independently in the UI
- Threads can be collapsed/expanded

### Thread Operations

- **Create Thread**: Create a reply thread under a message
- **Send Thread Message**: Send message within a thread
- **View Thread**: View all messages in a thread
- **Collapse/Expand Thread**: Control thread display state
- **Thread Notifications**: Configure whether to receive notifications for messages in the thread

## Message Mentions

Message mentions are used to notify specific users or groups.

### Mention Types

| Type | Description | Example |
|------|-------------|---------|
| **user** | Mention a specific user | @alice |
| **channel** | Mention all channel members | @everyone, @channel |
| **role** | Mention a specific role | @admin, @dev-team |

### Mention Properties

| Property | Description | Type |
|----------|-------------|------|
| **mention_type** | Mention type | user/channel/role |
| **mention_id** | Mentioned object ID | user_id or role_id |
| **mention_name** | Mention display name | String |
| **offset** | Starting position in text | Integer (optional, for text highlighting) |
| **length** | Length in text | Integer (optional, for text highlighting) |

### Mention Operations

- **Add Mention**: Add a mention to a message
- **Parse Mention**: Client parses mentions in message
- **Send Notification**: Mentioned users receive notifications
- **Highlight Display**: Mentioned parts are highlighted in the message

### Mention Content Structure Example

For text type messages, content may include:
```json
{
  "text": "Hi everyone, @alice please check this document",
  "mentions": [
    {
      "mention_type": "user",
      "mention_id": "user_123",
      "mention_name": "alice",
      "offset": 5,
      "length": 6
    }
  ]
}
```
