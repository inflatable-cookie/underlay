S3 adapters cache successful bucket and policy setup and redact provider details from storage errors. Verified upload finalisation enforces the upload size limit during reads. Custom blob adapters that use verified finalisation must implement bounded reads.

Media rendition generation uses Tokio's blocking pool, square thumbnails decode once, and media usage sync writes each desired edge at most once. Run asynchronous rendition generation on a Tokio runtime.

Polling job runners can stop after settling their current job with `run_until_shutdown`. Missing handlers emit failure and dead-letter events when the store reports a dead letter. PostgreSQL ignores stale failure attempts by returning a conflict, and outbox handlers have a configurable 30-second default deadline while their event is locked.

Email address formatting quotes and escapes display names where needed. Required SMTP TLS uses implicit TLS on port 465 and STARTTLS on other ports. SMTP failures are classified from structured errors. AI retries use capped jitter, and validation failures leave the provider circuit breaker state intact.
