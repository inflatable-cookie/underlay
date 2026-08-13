<script lang="ts">
  import { setContext } from "svelte";
  import { createToastStore, UNDERLAY_TOASTS_CONTEXT_KEY } from "../../src/patterns/toasts";
  import { UserActivityList, UserSessionsList } from "../../src/templates";
  import type {
    UserActivityItem,
    UserActivityListLoader,
    UserSessionItem,
    UserSessionListLoader
  } from "../../src/templates";

  interface Props {
    activityPaginated?: boolean;
  }

  let { activityPaginated = false }: Props = $props();

  setContext(UNDERLAY_TOASTS_CONTEXT_KEY, createToastStore());

  let sessionsActive = $state(false);
  let activityActive = $state(false);

  let sessions = $state<UserSessionItem[]>([
    {
      id: "session-1",
      status: "active",
      ipAddress: "203.0.113.10",
      userAgent: "Test Browser",
      createdAt: "2026-08-01T10:00:00Z",
      lastUsedAt: "2026-08-10T09:00:00Z"
    },
    {
      id: "session-2",
      status: "revoked",
      ipAddress: null,
      userAgent: null,
      createdAt: "2026-07-01T10:00:00Z",
      lastUsedAt: "2026-07-02T09:00:00Z"
    }
  ]);

  const activityEntries: UserActivityItem[] = [
    {
      id: "activity-1",
      occurredAt: "2026-08-10T09:00:00Z",
      action: "login",
      resourceType: "session",
      resourceId: "session-1",
      actor: { email: "admin@example.com" }
    },
    {
      id: "activity-2",
      occurredAt: "2026-08-09T09:00:00Z",
      action: "update",
      resourceType: "user",
      resourceId: null,
      actor: null
    },
    ...Array.from({ length: 43 }, (_, index) => ({
      id: `activity-${index + 3}`,
      occurredAt: "2026-08-08T09:00:00Z",
      action: "read",
      resourceType: "user",
      resourceId: `resource-${index + 3}`,
      actor: null
    }))
  ];

  let sessionRequests = $state<string[]>([]);
  let activityRequests = $state<string[]>([]);
  let sessionCount = $state(-1);
  let activityCount = $state(-1);
  let lastRevoked = $state("none");

  const loadSessions: UserSessionListLoader = async (_userId, _fetch, _token, request) => {
    sessionRequests = [...sessionRequests, `${request.page}:${request.limit}`];
    return { data: sessions.slice(0, request.limit), total: sessions.length };
  };

  const loadActivity: UserActivityListLoader = async (_userId, _fetch, _token, request) => {
    activityRequests = [...activityRequests, `${request.page}:${request.limit}`];
    const start = (request.page - 1) * request.limit;
    return {
      data: activityEntries.slice(start, start + request.limit),
      total: activityEntries.length
    };
  };

  async function revokeSession(session: UserSessionItem) {
    lastRevoked = session.id;
    sessions = sessions.map((entry) =>
      entry.id === session.id ? { ...entry, status: "revoked" } : entry
    );
  }
</script>

<button type="button" data-testid="activate-sessions" onclick={() => (sessionsActive = true)}>
  Activate sessions
</button>
<button type="button" data-testid="activate-activity" onclick={() => (activityActive = true)}>
  Activate activity
</button>

<UserSessionsList
  userId="user-1"
  active={sessionsActive}
  dataLoader={loadSessions}
  revokeAction={revokeSession}
  onCountChange={(count) => (sessionCount = count)}
/>

<UserActivityList
  userId="user-1"
  active={activityActive}
  paginated={activityPaginated}
  dataLoader={loadActivity}
  onCountChange={(count) => (activityCount = count)}
/>

<p data-testid="session-requests">{JSON.stringify(sessionRequests)}</p>
<p data-testid="activity-requests">{JSON.stringify(activityRequests)}</p>
<p data-testid="session-count">{sessionCount}</p>
<p data-testid="activity-count">{activityCount}</p>
<p data-testid="last-revoked">{lastRevoked}</p>
