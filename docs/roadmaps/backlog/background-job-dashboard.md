# Backlog: Background Job Dashboard

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 8-10 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

Can't see what background jobs are running or why they failed. Debugging failed jobs requires database queries and log diving.

---

## Proposed Solution

Web UI for monitoring background jobs, extending `underlay-jobs`.

### Potential Features

- List queued/running/failed jobs
- View job payloads and errors
- Retry failed jobs
- Cancel running jobs
- Job statistics and metrics
- Search/filter jobs
- Real-time updates (optional WebSocket)

### UI Mockup

```
+--------------------------------------------------+
| Background Jobs                    [Refresh] [?] |
+--------------------------------------------------+
| Filter: [All v] [Last 24h v] [Search...        ] |
+--------------------------------------------------+
| Status    | Job Type       | Created    | Action |
|-----------|----------------|------------|--------|
| Running   | send_email     | 2m ago     | Cancel |
| Failed    | process_image  | 5m ago     | Retry  |
| Completed | sync_data      | 10m ago    | View   |
| Queued    | generate_pdf   | 1m ago     | Cancel |
+--------------------------------------------------+
| Showing 1-20 of 156 jobs          [< 1 2 3 4 >] |
+--------------------------------------------------+
```

---

## Dependencies

- `underlay-jobs` crate (exists as skeleton)
- Admin authentication
- Database tables for job tracking

---

## When to Build

- Debugging failed jobs becomes painful
- Volume of background jobs increases
- Need visibility into job processing
- Multiple team members need job access

---

## Success Criteria

- [ ] Web UI shows job list with filtering
- [ ] Can view job details (payload, error, stack trace)
- [ ] Can retry failed jobs
- [ ] Can cancel queued/running jobs
- [ ] Job statistics visible (success rate, avg duration)
- [ ] Admin-only authentication
- [ ] Documentation

---

## Risks & Considerations

- Performance with large job queues
- Real-time updates add complexity
- Authentication/authorization needed
- UI framework choice (embed in existing admin?)

---

**Created**: 2026-01-12
