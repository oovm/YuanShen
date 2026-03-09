# Schedules

Schedules are time-driven task execution mechanisms in AI Company, used to automatically execute workflows or tasks at specified times or in fixed cycles.

## Core Concepts

### Schedule

A schedule is a container for time-driven tasks, defining the time rules for task execution and the content to be executed.

### Trigger Types

Schedules support multiple trigger types:

- **One-time Trigger**: Executes once at a specified specific time
- **Recurring Trigger**: Repeats execution in fixed cycles (e.g., daily, weekly, monthly, etc.)
- **Conditional Trigger**: Triggers based on a combination of time and other conditions

## Schedule Structure

A schedule contains the following key attributes:

| Attribute | Description |
|-----------|-------------|
| **Name** | The identifying name of the schedule |
| **Description** | Detailed explanation of the schedule |
| **Trigger Rule** | Time expression defining when the schedule triggers |
| **Execution Content** | The workflow or task to execute when the schedule triggers |
| **Associated Project** | The project the schedule belongs to (optional) |
| **Status** | The running status of the schedule (enabled/disabled) |

## Trigger Rules

Schedules use standard cron expressions or simplified time configurations to define trigger rules:

- **Cron Expression**: Powerful and flexible time expression that supports seconds, minutes, hours, days, months, and weeks
- **Simplified Configuration**:
  - Specific time each day
  - Specific day and time each week
  - Specific date and time each month
  - Custom intervals (e.g., every 2 hours)

## Execution Content

Schedules can trigger the following types of execution content:

- **Workflow**: Execute a complete workflow process
- **Single Task**: Execute a specific single task
- **Notification Reminder**: Send a schedule reminder notification

## Use Cases

Schedules are suitable for the following scenarios:

- **Periodic Reports**: Automatically generate project progress reports daily/weekly
- **Data Synchronization**: Periodically synchronize external system data
- **Backup Tasks**: Periodically execute data backups
- **Reminder Notifications**: Send reminders at important time points
- **Batch Processing**: Execute batch data processing during off-peak hours

## Relationships with Other Concepts

- **Project**: Schedules can be associated with a specific project and execute in the project context
- **Workflow**: Schedules are commonly used to trigger the execution of workflows
- **Task**: Schedules can also directly trigger the execution of single tasks

## Management Features

Schedules provide the following management features:

- **Enable/Disable**: Control whether the schedule runs at any time
- **Manual Trigger**: Support manual immediate execution of the schedule
- **Execution History**: Record each execution situation and result of the schedule
- **Failure Retry**: Automatic retry mechanism when execution fails
