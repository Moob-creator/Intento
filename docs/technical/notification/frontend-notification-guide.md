# Smart Reminder System - Frontend Integration Guide

## Overview

The Smart Reminder System provides automatic deadline reminders and daily task review notifications. This guide shows how to integrate these features into the Intento frontend.

## Available Commands

### 1. Send Custom Notification

Send a desktop notification to the user.

```typescript
import { invoke } from '@tauri-apps/api/core';

// Basic usage
await invoke('send_notification', {
  title: 'Task Complete',
  body: 'Your task has been completed successfully!',
});

// With notification type
await invoke('send_notification', {
  title: 'Deadline Alert',
  body: 'Task "Write report" is due in 2 hours',
  notificationType: 'deadline', // 'deadline' | 'dailyreview' | 'custom'
});
```

**Parameters:**
- `title` (string, required): Notification title
- `body` (string, required): Notification message
- `notificationType` (string, optional): Type of notification
  - `'deadline'` - For task deadline reminders
  - `'dailyreview'` - For daily review reminders
  - `'custom'` - For custom notifications (default)

**Returns:** `Promise<void>`

**Errors:** String error message if notification fails

### 2. Check Expiring Tasks

Manually trigger a check for tasks expiring within 24 hours and send notifications.

```typescript
// Check for expiring tasks and notify
const count = await invoke<number>('check_expiring_tasks');
console.log(`Found ${count} task(s) expiring soon`);

// Example: Add a button to let users manually check
async function checkDeadlines() {
  try {
    const count = await invoke<number>('check_expiring_tasks');
    if (count === 0) {
      console.log('No tasks expiring soon');
    } else {
      console.log(`Notified user about ${count} expiring task(s)`);
    }
  } catch (error) {
    console.error('Failed to check expiring tasks:', error);
  }
}
```

**Parameters:** None

**Returns:** `Promise<number>` - Count of expiring tasks found

**Errors:** String error message if query fails

### 3. Test Notification

Send a test notification to verify the system is working.

```typescript
// Test the notification system
await invoke('test_notification');
```

**Parameters:** None

**Returns:** `Promise<void>`

**Use Cases:**
- Settings page: "Test Notifications" button
- First-run setup: Verify notification permissions
- Debug: Check if notifications are working

## Automatic Features

The following features run automatically without frontend intervention:

### Hourly Deadline Reminders

The system automatically checks every hour (at minute 0) for tasks expiring within 24 hours.

**Behavior:**
- Runs: Every hour at :00 (e.g., 10:00, 11:00, 12:00)
- Checks: Tasks with deadlines in the next 24 hours
- Excludes: Completed tasks, deleted tasks
- Sends: One notification per expiring task

**Notification Format:**
```
Title: Task Deadline Reminder: [Task Title]
Body: Deadline: [Formatted Deadline]
      Priority: [High/Medium/Low]
```

### Daily Review Reminder

Reminds users to review their tasks every day at 6 PM.

**Behavior:**
- Runs: Daily at 18:00 (6:00 PM)
- Sends: Single notification

**Notification Format:**
```
Title: Daily Review Reminder
Body: Time to review your tasks for today!
```

## Integration Examples

### Example 1: Task Detail Page

Show a manual "Check Deadlines" button:

```typescript
import { invoke } from '@tauri-apps/api/core';

function TaskDetailPage() {
  const handleCheckDeadlines = async () => {
    try {
      const count = await invoke<number>('check_expiring_tasks');
      if (count > 0) {
        // Optionally show a toast
        showToast(`${count} task(s) are expiring soon`);
      }
    } catch (error) {
      console.error('Error checking deadlines:', error);
    }
  };

  return (
    <button onClick={handleCheckDeadlines}>
      Check Deadlines
    </button>
  );
}
```

### Example 2: Settings Page

Add notification testing:

```typescript
import { invoke } from '@tauri-apps/api/core';

function SettingsPage() {
  const testNotifications = async () => {
    try {
      await invoke('test_notification');
      showToast('Test notification sent!');
    } catch (error) {
      showError('Failed to send notification. Check permissions.');
    }
  };

  return (
    <div className="settings">
      <h3>Notifications</h3>
      <button onClick={testNotifications}>
        Test Notifications
      </button>
      <p>
        Make sure desktop notifications are enabled in your system settings.
      </p>
    </div>
  );
}
```

### Example 3: Task Completion

Send a custom notification when a task is completed:

```typescript
import { invoke } from '@tauri-apps/api/core';

async function completeTask(taskId: number) {
  // Update task in database
  await invoke('update_task', {
    id: taskId,
    status: 'done',
  });

  // Send completion notification
  await invoke('send_notification', {
    title: 'Task Completed! 🎉',
    body: 'Great job! You completed your task.',
    notificationType: 'custom',
  });
}
```

### Example 4: Dashboard Widget

Show count of expiring tasks:

```typescript
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';

function ExpiringTasksWidget() {
  const [expiringCount, setExpiringCount] = useState(0);

  useEffect(() => {
    loadExpiringTasks();
  }, []);

  const loadExpiringTasks = async () => {
    try {
      // Note: You could create a custom command to just get the count
      // without sending notifications
      const tasks = await invoke<Task[]>('list_tasks');
      const now = Date.now() / 1000;
      const in24Hours = now + 24 * 60 * 60;

      const expiring = tasks.filter(
        (t) => t.deadline && t.deadline > now && t.deadline <= in24Hours
      );

      setExpiringCount(expiring.length);
    } catch (error) {
      console.error('Failed to load expiring tasks:', error);
    }
  };

  return (
    <div className="widget">
      <h3>Expiring Soon</h3>
      <div className="count">{expiringCount}</div>
      <p>task(s) expiring in 24 hours</p>
    </div>
  );
}
```

## Notification Permissions

### macOS
Notifications require user permission. On first notification:
1. macOS shows a permission dialog
2. User must click "Allow"
3. Permissions can be managed in System Settings > Notifications

### Windows
Windows notifications typically work by default. Users can manage notification settings in:
- Windows Settings > System > Notifications

### Linux
Depends on the desktop environment. Most modern Linux desktops support notifications via:
- GNOME: Settings > Notifications
- KDE Plasma: System Settings > Notifications
- Others: Varies by DE

## Troubleshooting

### Notifications not appearing

**Check 1: Test notification**
```typescript
await invoke('test_notification');
```

**Check 2: System permissions**
- Verify notification permissions are enabled for Intento
- On macOS: System Settings > Notifications > Intento
- On Windows: Settings > System > Notifications

**Check 3: Do Not Disturb**
- Ensure Do Not Disturb mode is disabled
- On macOS: System Settings > Focus
- On Windows: Settings > Focus assist

### Error: "Failed to send notification"

This usually indicates:
1. Notification permissions denied
2. Notification service unavailable
3. Invalid notification parameters (empty title/body)

**Solution:**
```typescript
try {
  await invoke('send_notification', { title: 'Test', body: 'Test message' });
} catch (error) {
  // Show user-friendly error
  console.error('Notification error:', error);
  alert('Please enable notifications in system settings');
}
```

## Best Practices

### 1. Don't Over-Notify
- Use notifications sparingly
- Reserve for important events only
- Respect user attention

### 2. Clear Messaging
- Keep titles short and descriptive
- Provide context in the body
- Use action-oriented language

### 3. Handle Permissions Gracefully
```typescript
async function sendNotificationSafely(title: string, body: string) {
  try {
    await invoke('send_notification', { title, body });
  } catch (error) {
    // Log but don't crash
    console.warn('Notification failed:', error);
    // Optionally: Show in-app toast instead
  }
}
```

### 4. Provide In-App Alternatives
- Not all users enable notifications
- Provide in-app indicators for important events
- Example: Badge counts, toast messages, alert banners

## TypeScript Types

For better type safety, define these types:

```typescript
// types/notifications.ts

export type NotificationType = 'deadline' | 'dailyreview' | 'custom';

export interface SendNotificationParams {
  title: string;
  body: string;
  notificationType?: NotificationType;
}

// Usage
import { invoke } from '@tauri-apps/api/core';
import type { SendNotificationParams } from './types/notifications';

async function sendNotification(params: SendNotificationParams): Promise<void> {
  await invoke('send_notification', params);
}
```

## Summary

The Smart Reminder System provides:
- ✅ Automatic hourly deadline checks
- ✅ Daily review reminders at 6 PM
- ✅ Manual notification triggers
- ✅ Custom notification support
- ✅ Cross-platform desktop notifications

Use these features to keep users informed about their tasks without overwhelming them with notifications.
