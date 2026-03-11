# Specimen Dossier: Temporal

Status: Draft
Specimen: Temporal (Go/Rust/Java/TypeScript)
Owner:
Last updated: 2026-03-11
Scope: Durable execution and workflow orchestration

## 1) Why this specimen matters

Temporal represents a different paradigm from traditional job queues: "durable execution" where the platform handles reliability, retries, and state management. Originally from Uber's Cadence, it's gaining traction for complex business processes.

## 2) Product and era context

- **Launched**: 2020 (forked from Uber Cadence 2016)
- **Positioning**: "Durable execution platform"
- **Era**: Microservices complexity management (2020-2024)
- **Competition**: Cadence (predecessor), Conductor, Camunda
- **Adoption**: Growing in enterprise (Datadog, Stripe, Netflix)

## 3) Defining bets

1. **Durable execution** - Code as workflow, platform handles reliability
2. **Event sourcing** - All state changes recorded
3. **Deterministic replay** - Workflow code must be deterministic
4. **Multi-language SDKs** - Go, Rust, Java, TypeScript
5. **Visibility** - Query workflow state at any time

## 4) Standout strengths

- **Durable execution**: Survives process crashes, retries automatically
- **Saga pattern**: Long-running transactions with compensation
- **Human-in-the-loop**: Approval steps, timers, signals
- **Query**: Inspect running workflow state
- **Multi-language**: Same workflow callable from any SDK
- **Scalability**: Handles millions of concurrent workflows
- **Time travel testing**: Deterministic replay for testing

## 5) Chronic weaknesses and recurring costs

- **Complexity** - Steep learning curve
- **Determinism constraints** - No random, no time, no external calls
- **Infrastructure** - Requires Temporal server cluster
- **Operational overhead** - Cassandra/PostgreSQL + Elasticsearch
- **Overkill for simple jobs** - Heavy for basic background tasks

## 6) Core Concepts

### Workflows

Workflows are durable functions:

```typescript
// TypeScript SDK
async function processOrder(orderId: string): Promise<string> {
  // Each step is durably logged
  const payment = await executeActivity(processPayment, orderId);
  
  // Sleep is durable (survives crashes)
  await sleep('24 hours');
  
  const shipment = await executeActivity(shipOrder, orderId);
  
  return shipment.trackingId;
}
```

**Key insight**: The workflow code is replayed from history on recovery.

### Activities

Activities are the actual work units:
- Retry policies configurable per activity
- Can be external services
- Heartbeating for long-running tasks

### Signals and Queries

- **Signals**: Send data to running workflows (approval, cancellation)
- **Queries**: Read workflow state without mutation

## 7) Saga Pattern

Temporal enables saga pattern for distributed transactions:

```typescript
async function sagaWorkflow(order: Order) {
  const compensations = [];
  
  try {
    await executeActivity(reserveInventory, order);
    compensations.push(() => executeActivity(releaseInventory, order));
    
    await executeActivity(processPayment, order);
    compensations.push(() => executeActivity(refundPayment, order));
    
    await executeActivity(createShipment, order);
  } catch (e) {
    // Run compensations in reverse order
    for (const compensate of compensations.reverse()) {
      await compensate();
    }
    throw e;
  }
}
```

## 8) Comparison with Traditional Job Queues

| Aspect | Temporal | Sidekiq/BullMQ/Underlay |
|--------|----------|------------------------|
| Unit of work | Workflow (durably executed) | Job (dispatched to worker) |
| Failure handling | Automatic replay | Retry with backoff |
| State | Queryable at any time | Stored in job record |
| Long-running | First-class (timers, sleep) | Cron/scheduled jobs |
| Orchestration | Built-in (child workflows) | Manual/batch |
| Complexity | High | Lower |
| Use case | Complex business processes | Background tasks |

## 9) Project-relevant lessons

### Adopt carefully

- **Saga pattern** - For multi-step transactions needing compensation
- **Human-in-the-loop** - Approval workflows
- **Query capability** - Inspect job state dynamically

### Reject early

- **For simple background jobs** - Overkill for thumbnail generation, emails
- **As default job system** - Too complex for typical use cases
- **Tight integration** - Keep Underlay's job system simple

### Prototype before deciding

- **Migration workflows** - Underlay's migration framework could use saga pattern
- **Long-running processes** - When jobs need human approval

## 10) Comparison with Underlay

| Feature | Temporal | Underlay `underlay-jobs` |
|--------|----------|-------------------------|
| Durability | Survives process crashes | PostgreSQL persistence |
| Replay | Automatic from history | Retry from beginning |
| Orchestration | Child workflows, signals | Not built-in |
| Human-in-loop | First-class | Not built-in |
| Query state | Query API | Read from DB |
| Saga/compensation | Built-in | Manual implementation |
| Complexity | High | Lower |

**Finding**: Temporal solves different problems than Underlay's job system. Consider for migration framework.

## 11) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| docs.temporal.io | Official docs | v1.x | High | Excellent docs |
| GitHub temporalio/temporal | Source | main | High | MIT license |
| Temporal blog | Company | 2020-2024 | Medium | Use case studies |
| "Designing Data-Intensive Applications" (Martin Kleppmann) | Book | 2017 | High | Durable execution theory |

## 12) Open questions

- When does Temporal's complexity become worth it?
- How does performance compare to simpler job queues?
- What are the operational costs at scale?

## Next Task

Include in value track synthesis for:
1. Job orchestration and saga patterns
2. Long-running workflow patterns
3. Migration framework considerations
