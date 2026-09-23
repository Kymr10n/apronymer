# apronymer
A tool to create project and tool names as apronmyes based on related terms

## Behavioral Analytics / Logging

Every request to the backend is logged (via `tracing`) with structured fields for:

- `ip` – client IP address (`X-Forwarded-For` / `X-Real-IP` aware, for use behind Azure Container Apps ingress)
- `method` and `path`
- `user_agent`
- `referrer`

Requests with a missing or non-browser `User-Agent` (e.g. `curl`, `wget`, `python-requests`, generic
bots/crawlers) are logged at `warn` level with the message `🚨 Suspicious request`, while normal requests are
logged at `info` level with `📊 Request analytics`. Request frequency per client IP is already tracked and
enforced by the rate limiter (10 requests/minute by default; see `backend/src/rate_limiter.rs`), which also
logs a `warn` when a client exceeds the limit.

Because the backend logs go to stdout/stderr, Azure Container Apps automatically forwards them to the
configured Log Analytics workspace, making them queryable from Azure Monitor / Application Insights. Example
KQL queries against the `ContainerAppConsoleLogs_CL` table:

```kusto
// Suspicious (non-browser / unusual User-Agent) requests
ContainerAppConsoleLogs_CL
| where Log_s contains "Suspicious request"
| project TimeGenerated, Log_s

// High-frequency callers that hit the rate limit
ContainerAppConsoleLogs_CL
| where Log_s contains "Rate limit exceeded"
| project TimeGenerated, Log_s
```

These can be turned into Azure Monitor alert rules to notify on abnormal traffic patterns.
