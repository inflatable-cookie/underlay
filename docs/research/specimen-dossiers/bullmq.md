# Specimen Dossier: BullMQ

Status: Draft
Specimen: BullMQ (Node.js/TypeScript)
Owner:
Last updated: 2026-03-11
Scope: Modern job queue with observability and flow orchestration

## 1) Why this specimen matters

BullMQ is the leading Node.js job queue, offering modern TypeScript-first APIs, excellent observability, and innovative "flows" for job orchestration. It represents the state-of-the-art for Redis-based job queues.

## 2) Product and era context

- **Launched**: 2019 (Bull v1 2015, BullMQ v1 2020)
- **Positioning**: "The fastest, most reliable, Redis-based queue for Node"
- **Era**: Modern Node.js/TypeScript (2020-2024)
- **Competition**: Bee Queue, Agendajs, node-resque
- **Adoption**: Very high in Node.js ecosystem

## 3) Defining bets

1. **TypeScript-first** - Full type safety
2. **Redis Streams** - Modern Redis features for reliability
3. **Flows** - Job dependencies and DAGs
4. **Sandboxed processors** - Process isolation for safety
5. **Observability-first** - Events, metrics, progress tracking

## 4) Standout strengths

- **Type safety**: Full TypeScript support with generics
- **Flows**: Parent-child job relationships, DAGs, dependencies
- **Sandboxed workers**: Process isolation prevents crashes
- **Rate limiting**: Token bucket and fixed window
- **Job progress**: Built-in progress reporting
- **Repeatable jobs**: Cron expressions with timezone support
- **Pause/resume**: Queue management at runtime
- **UI**: Bull Board for monitoring

## 5) Chronic weaknesses and recurring costs

- **Redis dependency** - Same as Sidekiq
- **Node.js only** - Not usable directly by Rust
- **Memory usage** - Redis memory can grow with large queues
- **No exactly-once** - At-least-once delivery
- **Stalled job detection** - Requires careful configuration

## 6) Flows (Job Orchestration)

BullMQ's most innovative feature is "Flows" - job dependencies:

```typescript
// Define a DAG of jobs
const flow = new FlowProducer();

await flow.add({
  name: "process-video",
  queueName: "video",
  children: [
    { name: "extract-audio", queueName: "audio", opts: { priority: 1 } },
    { name: "generate-thumbnail", queueName: "images" },
    { name: "transcode", queueName: "video", opts: { priority: 2 } }
  ]
});
```

**Use cases**:
- Video processing pipelines
- Data ETL workflows
- Multi-step business processes

**Comparison**: Underlay has no built-in job dependency/orchestration.

## 7) Retry and Backoff

BullMQ offers flexible backoff strategies:

```typescript
new Queue('my-queue', {
  defaultJobOptions: {
    attempts: 3,
    backoff: {
      type: 'exponential',
      delay: 1000
    }
  }
});
```

**Backoff types**:
- `fixed` - Constant delay
- `exponential` - Exponential with optional jitter
- Custom function - Full control

## 8) Observability Features

BullMQ is designed for observability:

```typescript
// Events for monitoring
queue.on('completed', (job) => {
  metrics.increment('jobs.completed');
});

queue.on('failed', (job, err) => {
  metrics.increment('jobs.failed');
  logger.error({ jobId: job.id, error: err });
});

// Progress tracking
job.updateProgress(42);
```

**Metrics available**:
- Queue depth (waiting, active, completed, failed)
- Job duration
- Retry counts
- Worker concurrency

## 9) Project-relevant lessons

### Adopt carefully

- **Progress tracking** - Built-in `JobProgress` in Underlay but needs usage patterns
- **Flows/DAGs** - Job orchestration is a real need; Underlay has nothing here
- **Sandboxed workers** - Process isolation for safety
- **Type safety** - Generics for job payloads

### Reject early

- **Redis requirement** - Underlay uses PostgreSQL intentionally
- **Node.js-only** - Not directly applicable to Rust

### Prototype before deciding

- **Flow orchestration in Rust** - Would require significant design work
- **Job progress patterns** - How should apps report progress?

## 10) Comparison with Underlay

| Feature | BullMQ | Underlay `underlay-jobs` |
|--------|--------|-------------------------|
| Orchestration | Flows (DAGs) | Not built-in |
| Progress tracking | `job.updateProgress()` | `JobProgress` type |
| Rate limiting | Built-in | Not built-in |
| Sandboxed workers | Process isolation | Thread-based |
| Type safety | Full TypeScript | Rust types |
| UI | Bull Board | App must build |
| Events | Rich event system | Limited |

**Finding**: Underlay lacks job orchestration (flows) and advanced rate limiting.

## 11) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| docs.bullmq.io | Official docs | v5.x | High | Excellent docs |
| GitHub taskforcesh/bullmq | Source | main | High | MIT license |
| Bull Board | UI | Current | High | Open source UI |

## 12) Open questions

- How do BullMQ flows handle partial failures?
- What's the performance overhead of sandboxed workers?
- How does BullMQ handle Redis cluster mode?

## Next Task

Include in value track synthesis focusing on:
1. Job orchestration patterns (flows vs manual)
2. Observability and progress tracking
3. Rate limiting strategies
