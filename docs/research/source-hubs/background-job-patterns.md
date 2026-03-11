# Source Hub: Background Job Patterns

Status: Draft
Hub: JOBS-001
Owner:
Last updated: 2026-03-11
Scope: Job queue reliability, retry patterns, observability, and execution models

## 1) Questions this hub should answer

- What are the common failure modes in job queue systems?
- How do successful job systems handle retries and dead letter queues?
- What observability patterns are essential for production job systems?
- How do different storage backends (Redis, PostgreSQL, SQS) compare?
- What are the tradeoffs between immediate execution vs durable scheduling?

## 2) Strongest primary sources

| Source family | Authority | Version/Currency | Biases or gaps | Notes |
| --- | --- | --- | --- | --- |
| Sidekiq (Ruby) | Mike Perham | v7.x (stable) | Ruby-centric | Industry standard, proven patterns |
| BullMQ (Node.js) | Taskforce.sh | v2.x (active) | Redis-only | Modern, TypeScript, excellent observability |
| Temporal (Go) | Temporal.io | v1.x (stable) | Complex learning curve | Durable execution, saga patterns |
| Faktory | Contribsys | v1.x (stable) | Language-agnostic | Transaction safety focus |
| PostgreSQL queue patterns | Various | Mature | PostgreSQL only | SKIP LOCKED, advisory locks |
| AWS SQS docs | Amazon | Current | AWS ecosystem | Managed service patterns |
| Celery (Python) | Open source | v5.x (stable) | Python-centric | Mature, many backends |

## 3) Secondary sources worth using carefully

| Source family | Why it helps | Risks or bias | Notes |
| --- | --- | --- | --- |
| Resque (Ruby) | Historical patterns | Legacy, less maintained | Sidekiq predecessor |
| Delayed Job (Ruby) | Simplicity | Limited features | Alternative to Sidekiq |
| RabbitMQ tutorials | Queue patterns | Erlang-centric, complex | Good for pattern understanding |
| Kafka streams | Event sourcing | Different use case | Not traditional job queue |
| Hangfire (.NET) | Dashboard patterns | .NET ecosystem | Good UI reference |

## 4) Source rules

1. **Reliability over speed** - Durable execution matters more than throughput
2. **Observability is essential** - Can't debug what you can't see
3. **Retry storms are real** - Exponential backoff with jitter is critical
4. **Poison pills happen** - Dead letter queues are not optional
5. **Queue depth matters** - Backpressure and alerting essential

## 5) Tracks or questions this hub should feed

- Value Track: Retry and failure handling patterns
- Value Track: Observability and monitoring for job systems
- Specimen Dossier: Sidekiq (reliability patterns)
- Specimen Dossier: BullMQ (modern observability)
- Specimen Dossier: Temporal (durable execution)

## 6) Known blind spots

- Underlay's PostgreSQL-based approach vs Redis performance characteristics
- WebSocket/real-time job progress updates
- Multi-region job execution patterns
- Job queue backpressure strategies

## Next Task

Create specimen dossiers for Sidekiq (reliability patterns), BullMQ (modern observability), and Temporal (durable execution).
