# Specimen Dossier: Sidekiq

Status: Draft
Specimen: Sidekiq (Ruby)
Owner:
Last updated: 2026-03-11
Scope: Background job processing reliability patterns

## 1) Why this specimen matters

Sidekiq is the most widely deployed background job system, processing billions of jobs daily. Its reliability patterns (retries, dead letters, batching) are battle-tested in production environments for over a decade.

## 2) Product and era context

- **Launched**: 2012 by Mike Perham
- **Positioning**: "Simple, efficient background processing for Ruby"
- **Era**: Post-Resque, Redis-based job queues (2012-2024)
- **Competition**: Resque (predecessor), Delayed Job, Celery, BullMQ
- **Adoption**: Ubiquitous in Ruby ecosystem, processing billions of jobs

## 3) Defining bets

1. **Redis as backend** - Fast, simple, well-understood
2. **Threads over processes** - Memory efficiency
3. **JSON serialization** - Human-readable, debuggable
4. **Middleware pattern** - Extensible job lifecycle
5. **Retries with exponential backoff** - Automatic recovery

## 4) Standout strengths

- **Reliability**: Battle-tested at massive scale (GitHub, Shopify, etc.)
- **Retry system**: Exponential backoff with jitter, configurable per job
- **Dead letter queue**: Failed jobs retained for inspection
- **Batch processing**: Group jobs, track completion
- **Scheduled jobs**: Cron-like scheduling built-in
- **Web UI**: Excellent dashboard for monitoring and management
- **Pro/Enterprise features**: Batches, unique jobs, rate limiting, encryption

## 5) Chronic weaknesses and recurring costs

- **Redis dependency** - Single point of failure, memory limitations
- **Ruby ecosystem** - Not directly usable by Rust/TypeScript
- **No exactly-once semantics** - At-least-once delivery only
- **Job loss risk** - Redis persistence modes matter
- **Memory bloat** - Large job payloads problematic

## 6) Retry Pattern Deep Dive

Sidekiq's retry system is the gold standard:

```ruby
# Default: 25 retries over ~21 days
# Backoff: exponential with jitter
# Formula: count**4 + 15 + (rand(30) * (count + 1))
```

Retry schedule (first 10):
1. 0 seconds (immediate)
2. 16 seconds
3. 1 minute
4. 4 minutes
5. 12 minutes
6. 29 minutes
7. 1 hour
8. 2 hours
9. 4 hours
10. 8 hours

**Key insights**:
- Jitter prevents thundering herd
- 25 retries = ~21 days total
- Eventually gives up (dead letter)
- Configurable per job class

## 7) Dead Letter Pattern

Sidekiq's "morgue" (dead letter queue):
- Failed jobs retained after retries exhausted
- Can be retried manually via UI
- Configurable retention period
- Separate from active queues

## 8) Middleware Pattern

Sidekiq's middleware allows custom job lifecycle hooks:

```ruby
class MyMiddleware
  def call(worker, job, queue)
    # Before job execution
    yield
    # After successful execution
  rescue => e
    # On failure
    raise
  end
end
```

**Use cases**:
- Logging and metrics
- Request context propagation
- Circuit breakers
- Rate limiting

## 9) Project-relevant lessons

### Adopt carefully

- **Exponential backoff with jitter** - Critical for retry storms
- **Dead letter queue** - Essential for debugging failures
- **Middleware pattern** - Clean extensibility
- **Job config per type** - Different jobs need different policies

### Reject early

- **Redis as primary store** - Underlay uses PostgreSQL; different tradeoffs
- **25 default retries** - Too many for most use cases; 3-5 is usually enough
- **Ruby-specific patterns** - Some patterns don't translate directly

### Prototype before deciding

- **PostgreSQL vs Redis performance** - Measure actual throughput needs
- **Job progress tracking** - Sidekiq batches pattern

## 10) Comparison with Underlay

| Feature | Sidekiq | Underlay `underlay-jobs` |
|--------|---------|-------------------------|
| Storage | Redis | PostgreSQL |
| Retry backoff | Exponential with jitter | Configurable (None, Linear, Exponential) |
| Dead letter | Morgue | `Failed` status |
| Web UI | Excellent | App must build |
| Batch processing | Pro feature | Not built-in |
| Scheduled jobs | Cron-like | `scheduler` feature |
| Middleware | Yes | `JobHandler` trait |
| Unique jobs | Pro feature | `allow_overlap: false` |

**Finding**: Underlay has the basics but lacks advanced observability and batching.

## 11) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| sidekiq.org | Official docs | v7.x | High | Excellent documentation |
| GitHub mperham/sidekiq | Source | main | High | LGPL license |
| Sidekiq wiki | Community | Current | High | Best practices |
| "The quieter you become..." (Mike Perham blog) | Author | 2012-2024 | High | Design philosophy |

## 12) Open questions

- How does Sidekiq Pro's batch processing work at scale?
- What are the memory characteristics of large job payloads?
- How does Sidekiq handle Redis failover?

## Next Task

Create value track synthesis comparing retry patterns, observability approaches, and batch processing across Sidekiq, BullMQ, and Temporal.
