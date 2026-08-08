# Backlog: Real-time / WebSocket Layer

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 15-20 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

Need real-time updates for features like chat, notifications, presence tracking, and collaborative editing.

---

## Proposed Solution

WebSocket infrastructure with channels and presence.

### Potential Features

- Connection management (connect, disconnect, reconnect)
- Channel subscriptions (join, leave, broadcast)
- Presence tracking (who's online, typing indicators)
- Message persistence (optional)
- Fallback for older browsers (long polling)

### Server-Side Design

```rust
use underlay_realtime::{Channel, Presence, Message};

// Create a channel
let channel = Channel::new("room:123");

// Broadcast to all subscribers
channel.broadcast(Message::new("user_joined", json!({ "user_id": "abc" }))).await;

// Track presence
channel.presence.track(user_id, json!({ "name": "Alice" })).await;
let online_users = channel.presence.list().await;
```

### Client-Side Design

```typescript
import { createSocket } from '@inflatable-cookie/underlay/realtime';

const socket = createSocket({ url: '/ws', token: authToken });

const channel = socket.channel('room:123');
await channel.join();

channel.on('user_joined', (payload) => {
  console.log('User joined:', payload.user_id);
});

channel.presence.onSync((presences) => {
  console.log('Online users:', presences);
});
```

---

## Dependencies

- WebSocket server infrastructure
- Authentication integration
- Scaling strategy (sticky sessions or Redis pub/sub)

---

## When to Build

- Users requesting live updates
- Polling becoming excessive
- Collaboration features needed
- Notification feeds required

---

## Success Criteria

- [ ] WebSocket connection management with reconnection
- [ ] Channel-based subscriptions
- [ ] Broadcast and direct messaging
- [ ] Presence tracking
- [ ] Authentication per connection
- [ ] Works with load balancers
- [ ] Client SDK for Svelte
- [ ] Documentation and examples

---

## Risks & Considerations

- Infrastructure complexity (WebSocket support, scaling)
- Sticky sessions or shared state needed for multiple servers
- Memory usage for many connections
- Authentication and authorization per message
- Fallback for environments without WebSocket

---

**Created**: 2026-01-12
