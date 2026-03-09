# User Profile

User profiles describe a user's public information, including avatar, nickname, bio, presence status, and more.

## Profile Properties

| Property | Description | Type |
|----------|-------------|------|
| **user_id** | User ID | user_id |
| **subnet_id** | Subnet ID | subnet_id |
| **avatar** | Avatar | Resource reference or URL (optional) |
| **nickname** | Nickname | Display name within the subnet (optional) |
| **bio** | Biography | String (optional) |
| **status_text** | Custom status text | String (optional, e.g., "In a meeting", "Out to lunch") |
| **presence_status** | Presence status | Enum (online/busy/away/offline, optional) |
| **last_active_at** | Last active timestamp | Timestamp (optional) |
| **updated_at** | Profile update timestamp | Timestamp (optional) |
| **device_info** | Device information | JSON object (optional, e.g., device type, operating system, etc.) |

## Presence Status Types

| Status | Description | Icon |
|--------|-------------|------|
| **online** | User is currently active and using the client | 🟢 |
| **busy** | User is online but busy and may not respond promptly | 🔴 |
| **away** | User is online but has been away for some time (e.g., 5 minutes of inactivity) | 🟡 |
| **offline** | User is not online or has no active connection | ⚪ |

## Presence Status Update Mechanism

### Client-Triggered Updates

- **Online**: Automatically set to "online" when the client connects to a service node
- **Active**: Update last active timestamp when the user performs actions (sending messages, viewing channels, etc.)
- **Away**: Automatically set to "away" when the client detects user inactivity for a period (e.g., 5 minutes)
- **Busy**: User can manually set to "busy" status
- **Offline**: Automatically set to "offline" when the client disconnects

### Service Node Management

- Service nodes maintain user presence status
- Detect user online status via heartbeat mechanism
- Broadcast status changes to relevant users (e.g., members in the same channel)

## Profile Visibility

- User profiles are publicly visible within the subnet
- Configurable who can view profile information:
  - Everyone
  - Contacts only
  - Same channel members only
  - Custom
- Sensitive fields (e.g., device_info) can have separate visibility configurations

## Profile Operations

- **View Profile**: View other users' profiles
- **Update Profile**: Update your own avatar, nickname, and bio
- **Set Presence Status**: Manually set your presence status (online/busy/away)
- **Set Status Text**: Set custom status text
- **Clear Status**: Clear custom status and use default status

## Notification Mechanism

- Optionally notify relevant users when a user's profile changes
- Support for toggling status change notifications
- Configurable to follow profile changes for specific users

## Privacy Considerations

- User profiles are public but do not contain sensitive information
- Option to hide presence status
- Device information is optional; users can choose not to share it
- Support for partial profile hiding (e.g., show only nickname, not avatar)
