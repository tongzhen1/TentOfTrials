# Operations Guide


> WARNING: This operations guide is a LEGACY document. It was last updated
> when the system was running on bare-metal servers in a colocation facility.
> The system has since been migrated to Kubernetes on AWS EKS. Some of the
> commands and procedures in this document are specific to the old infrastructure
> and will not work in the current environment. The Kubernetes-specific operations
> are documented in the internal wiki under "Kubernetes Operations."
>
> The migration from bare-metal to Kubernetes was completed in Q2 2023 but
> this document was never updated because the operations team was busy with
> the post-migration stability work. The post-migration work is still ongoing.
> The known issues from the migration are tracked in the "K8s Migration Known
> Issues" spreadsheet which is linked from the team's shared drive.


## Monitoring

## Backend Environment Configuration

The Rust backend reads the following environment variables at startup. All
variables are optional and have safe local-development defaults.

| Variable | Default | Description |
|----------|---------|-------------|
| `TOT_BACKEND_HOST` | `0.0.0.0` | Bind address used by the backend service. |
| `TOT_BACKEND_PORT` | `8080` | Backend service port. Must be a valid TCP port from 1 to 65535. |
| `TOT_LOG_LEVEL` | `info` | Log filter passed to `tracing_subscriber`, for example `debug` or `warn`. |
| `TOT_ENABLE_EXPERIMENTAL` | `false` | Enables guarded experimental backend behavior when set to `true`. |

Invalid `TOT_BACKEND_PORT` and `TOT_ENABLE_EXPERIMENTAL` values fail startup
with descriptive errors so misconfigured deployments do not silently fall back
to unsafe assumptions.


### Health Check Endpoints


Each service exposes a health check endpoint:


| Service | Endpoint | Port |
|---------|----------|------|
| Backend API | `/health` | 8080 |
| Market Engine | `/health` | 8081 |
| Frailbox Runtime | `/health` | 8082 |
| Frontend | `/` | 3000 |


The health check returns a 200 OK response with a JSON body:


```json
{
  "status": "ok",
  "version": "3.2.0",
  "uptime_seconds": 86400,
  "timestamp": "2024-01-15T00:00:00Z"
}
```


### Prometheus Metrics


Each service exposes Prometheus metrics at `/metrics` on the same port as the
health check endpoint. The metrics are scraped by the Prometheus server every
15 seconds.


Key metrics to monitor:


| Metric | Type | Description | Warning Threshold | Critical Threshold |
|--------|------|-------------|-------------------|-------------------|
| `http_requests_total` | Counter | Total HTTP requests | - | - |
| `http_request_duration_ms` | Histogram | Request latency | p99 > 500ms | p99 > 2000ms |
| `http_errors_total` | Counter | HTTP error responses | > 1% of requests | > 5% of requests |
| `active_connections` | Gauge | Active connections | > 80% of max | > 95% of max |
| `memory_usage_bytes` | Gauge | Process memory | > 80% of limit | > 90% of limit |
| `cpu_usage_percent` | Gauge | CPU usage | > 70% | > 90% |
| `db_connection_pool_size` | Gauge | Database connections | > 80% of pool | > 95% of pool |
| `queue_depth` | Gauge | Message queue depth | > 1000 | > 10000 |
| `goroutine_count` | Gauge | Go routine count | > 5000 | > 10000 |
| `gc_pause_time_ms` | Histogram | GC pause time | > 100ms | > 500ms |


### Grafana Dashboards


Pre-built Grafana dashboards are available:


| Dashboard | Description | UID |
|-----------|-------------|-----|
| System Overview | CPU, memory, disk, network | `tot-system-overview` |
| API Performance | Request latency, throughput, errors | `tot-api-performance` |
| Market Data | Order book, trade volume, spread | `tot-market-data` |
| Business Metrics | Active users, trades, volume | `tot-business-metrics` |
| Service Health | Per-service health and dependencies | `tot-service-health` |


### Alerting Rules


Alerts are sent to PagerDuty and Slack (#ops-alerts channel).
