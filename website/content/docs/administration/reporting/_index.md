---
title: Reporting
linkTitle: Reporting
description: "Configure Couic telemetry reporting via webhooks. Send filtering events to external endpoints for monitoring and alerting."
keywords: ["Couic", "reporting", "telemetry", "webhook", "notifications"]
images: ["/images/couic-og.png"]
prev: /docs/administration/peering
next: /docs/administration/reporting/couic-report
weight: 5
---

Couic provides a telemetry reporting mechanism that enables centralized monitoring and analysis of filtering actions. This feature allows Couic instances to send structured reports about policy enforcement to a local or remote webhook endpoint for aggregation, alerting, and incident response.

Couic internally batches report transmissions at fixed **500 ms intervals**. This buffering mechanism ensures efficient delivery without overwhelming the reporting endpoint. The system is designed to handle high-volume scenarios during DDoS attacks, where thousands of events per second may need to be reported while maintaining system stability and preventing memory exhaustion.

{{< callout type="info" >}}
The reporting feature is optional and disabled by default. Reports are sent asynchronously and do not block firewall operations.
{{< /callout >}}

{{< callout type="info" >}}Couic provides a ready-to-use reporting endpoint implementation called `couic-report`. For production deployments, see the [couic-report documentation](/docs/administration/reporting/couic-report).
{{</ callout >}}

## Architecture

The reporting system consists of three main components:

1. **Report Generation**: Each time a rule is created, a structured report is created containing the action, policy, network entry, and optional metadata.

2. **Buffered Channel**: Reports are queued in a channel. If the buffer is full, new reports are dropped with a warning to prevent memory exhaustion.

3. **Worker Thread**: A dedicated background thread batches and sends reports to the configured webhook endpoint at 500 ms intervals using HTTP POST requests.

{{< callout type="info" >}}
When the reporting endpoint is unreachable or returns errors, the worker implements an **exponential backoff** strategy to prevent aggressive retries during outages while ensuring eventual delivery when the endpoint recovers.
{{</ callout >}}

## Report Structure

Each report contains the following fields:

```json
{
  "action": "add",
  "policy": "drop",
  "entry": {
    "creation": 1234567880,
    "cidr": "1.2.3.4/32",
    "tag": "scanner",
    "expiration": 1234567890
  },
  "metadata": {
    "kind": "manual",
    "detail": "Port scanning detected",
    "extra": {
      "source_port": 12345,
      "scan_type": "SYN",
      "attempts": 156
    }
  }
}
```

{{< callout type="important" >}}
The webhook endpoint receives an **array of reports**, not individual reports. Couic batches multiple reports together and sends them as a JSON array in a single HTTP POST request every 500 ms.
{{< /callout >}}

### Field Descriptions

- **action**: The synchronization action performed. Limited to the following enum values:
  - `add`: A new filtering rule was created
  - `remove`: An existing filtering rule was deleted

- **policy**: The enforcement policy applied. Limited to the following enum values:
  - `drop`: Traffic is dropped (blocked)
  - `ignore`: Traffic is allowed (whitelisted)

- **entry**: Details of the network entry being filtered:
  - **creation**: Unix timestamp (seconds) when the rule was created
  - **cidr**: Network address in CIDR notation (e.g., `192.168.1.0/24`)
  - **tag**: Optional label for categorization (e.g., `scanner`, `malicious`, `set.couic`)
  - **expiration**: Unix timestamp (seconds) when the rule expires, or `0` for permanent rules

- **metadata**: Optional contextual information to enrich the report. This field is **completely flexible** and can contain any JSON-serializable data structure relevant to your monitoring needs:
  - **kind**: Classification or source type (e.g., `manual`, `automated`, `feed`, `reputation`)
  - **detail**: Human-readable description of the event
  - **extra**: Arbitrary JSON object with additional context-specific fields

{{< callout type="info" >}}
The metadata field allows you to attach custom information such as:
- Threat intelligence data (IOC type, confidence score, MITRE ATT&CK techniques)
- Detection context (sensor ID, alert severity, correlation ID)
- Attack patterns (protocol, ports, payloads, signatures)
- Geographic data (country, ASN, organization)
- Custom business logic fields specific to your deployment
{{< /callout >}}

## Configuration

To enable reporting, add the following in Couic configuration:

```toml {filename="/etc/couic/couic.toml"}
[reporting]
enabled = true
webhook = "https://telemetry.example.com/v1/reports"
```

### Configuration Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `enabled` | boolean | Yes | Enables or disables the reporting feature |
| `webhook` | string | Yes | HTTP(S) endpoint URL to receive reports |

### HTTP Client Configuration

The reporting client is configured with:
- **Timeout**: 2 seconds per request
- **User-Agent**: `couic/<version>`
- **Content-Type**: `application/json`
- **Accept**: `application/json`

## Setting Up a Reporting Endpoint

Your webhook endpoint must accept HTTP POST requests containing a **JSON array of reports**. Each batch may contain anywhere from a single report to thousands of reports, depending on the current filtering activity.

```http
POST /v1/reports HTTP/1.1
Host: telemetry.example.com
Content-Type: application/json
User-Agent: couic/1.0.0
Accept: application/json

[
  {
    "action": "add",
    "policy": "drop",
    "entry": {
      "creation": 1704067200,
      "cidr": "1.2.3.4/32",
      "tag": "malicious",
      "expiration": 1704070800
    },
    "metadata": {
      "kind": "automated",
      "detail": "Repeated connection attempts"
    }
  },
  {
    "action": "add",
    "policy": "ignore",
    "entry": {
      "creation": 1704067205,
      "cidr": "5.6.7.8/32",
      "tag": "trusted",
      "expiration": 0
    }
  },
  {
    "action": "remove",
    "policy": "drop",
    "entry": {
      "creation": 1704067100,
      "cidr": "9.10.11.12/32",
      "tag": "scanner",
      "expiration": 1704067210
    },
    "metadata": {
      "kind": "manual",
      "detail": "Rule expired"
    }
  }
]
```

The endpoint should return:
- **200 OK** or **2xx** status codes for successful processing
- **4xx/5xx** status codes for errors (triggers retry with backoff)

## Security Considerations

### Authentication

The webhook URL should include authentication credentials:
- **UUID in path**: `https://telemetry.example.com/v1/reports/{uuid}`
- **API key in header**: Configure via reverse proxy
- **mTLS**: For production deployments

### Data Privacy

Reports may contain sensitive information:
- IP addresses of filtered hosts
- Attack patterns and timestamps
- Consider GDPR/privacy compliance requirements

### TLS/HTTPS

Always use HTTPS endpoints in production to ensure:
- Data confidentiality during transmission
- Authentication of the reporting endpoint
- Integrity of report data

## Example: Complete Setup

{{% steps %}}

### Configure Couic

```toml {filename="/etc/couic/couic.toml"}
[reporting]
enabled = true
webhook = "https://telemetry.example.com/v1/reports/550e8400-e29b-41d4-a716-446655440000"
```

### Restart Couic

```bash
systemctl restart couic
```

### Trigger a test report

```bash
couicctl drop add 1.2.3.4/32 -e 30s -t test-report
```

### Verify delivery in your endpoint logs

```txt
INFO: Received 1 reports from Couic
INFO: Action: add, Policy: drop, CIDR: 1.2.3.4/32, Tag: test-report
```
{{% /steps %}}


## Integration with Monitoring Systems

The reporting feature integrates well with:
- **Prometheus/Grafana**: Parse reports and expose metrics
- **Elasticsearch/Kibana**: Store and visualize filtering events
- **Splunk**: Centralized log aggregation and analysis
- **Custom SIEM**: Security incident correlation and response
