---
title: Monitoring
linkTitle: Monitoring
description: "Monitor Couic with Prometheus — Export XDP packet counters, CIDR statistics, per-tag metrics, and build Grafana dashboards"
keywords: ["Couic", "monitoring", "Prometheus", "Grafana", "XDP metrics", "observability", "firewall statistics"]
images: ["/images/couic-og.png"]
prev: /docs/administration/reverse-proxy
next: /docs/administration/peering
weight: 3
---

Couic’s statistics can be accessed in [Prometheus format](https://prometheus.io/docs/concepts/data_model/) for monitoring purposes.

{{< callout type="info" >}}
This section assumes that remote access to the Couic API has already been set up using a reverse proxy.
For instructions on enabling remote access, see the [Reverse Proxy section](/docs/administration/reverse-proxy) first.
{{< /callout >}}

## Add a monitoring client

Add a new client to `monitoring` group:

```bash {filename="command"}
couicctl clients add -n prometheus -g monitoring
```

```bash {filename="output"}
┌─────────────┬────────────┬──────────────────────────────────────┐
│ Name        ┆ Group      ┆ Token                                │
╞═════════════╪════════════╪══════════════════════════════════════╡
│ prometheus  ┆ monitoring ┆ d6ac883a-8050-4408-bf1e-5b07e9965191 │
└─────────────┴────────────┴──────────────────────────────────────┘
```

{{< callout type="info" >}}
For more details about client permissions see [Authentication and Authorization](/docs/administration/auth).
{{< /callout >}}

## Test using curl

Test using curl and `couicctl` token:

```bash {filename="command"}
curl "https://couic.tld:2900/v1/metrics?format=prometheus" \
    -H "Authorization: Bearer d6ac883a-8050-4408-bf1e-5b07e9965191"
```

```txt {filename="output"}
# HELP couic_drop_cidr_total Current number of CIDR dropped by couic.
# TYPE couic_drop_cidr_total gauge
couic_drop_cidr_total 3
# HELP couic_ignore_cidr_total Current number of CIDR ignored by couic.
# TYPE couic_ignore_cidr_total gauge
couic_ignore_cidr_total 0
# HELP couic_stats_rx_packets_total Current number of packets handled by XDP.
# TYPE couic_stats_rx_packets_total counter
couic_stats_rx_packets_total{action="XDP_ABORTED"} 0
couic_stats_rx_packets_total{action="XDP_DROP"} 29231
couic_stats_rx_packets_total{action="XDP_REDIRECT"} 0
couic_stats_rx_packets_total{action="XDP_PASS"} 586018
couic_stats_rx_packets_total{action="XDP_TX"} 0
# HELP couic_stats_rx_bytes_total Current number of bytes handled by XDP.
# TYPE couic_stats_rx_bytes_total counter
couic_stats_rx_bytes_total{action="XDP_ABORTED"} 0
couic_stats_rx_bytes_total{action="XDP_DROP"} 2360999
couic_stats_rx_bytes_total{action="XDP_REDIRECT"} 0
couic_stats_rx_bytes_total{action="XDP_PASS"} 46562369
couic_stats_rx_bytes_total{action="XDP_TX"} 0
# HELP couic_drop_tag_rx_packets_total Number of packets dropped per tag.
# TYPE couic_drop_tag_rx_packets_total counter
couic_drop_tag_rx_packets_total{tag="fail2ban-sshd"} 20981
# HELP couic_drop_tag_rx_bytes_total Number of bytes dropped per tag.
# TYPE couic_drop_tag_rx_bytes_total counter
couic_drop_tag_rx_bytes_total{tag="fail2ban-sshd"} 1714333
# HELP couic_ignore_tag_rx_packets_total Number of packets ignored per tag.
# TYPE couic_ignore_tag_rx_packets_total counter
# HELP couic_ignore_tag_rx_bytes_total Number of bytes ignored per tag.
# TYPE couic_ignore_tag_rx_bytes_total counter
# EOF
```

The `/v1/metrics` endpoint also supports JSON output (default):

```bash {filename="command"}
curl "https://couic.tld:2900/v1/metrics" \
    -H "Authorization: Bearer d6ac883a-8050-4408-bf1e-5b07e9965191"
```

```json {filename="output"}
{
  "drop_cidr_count": 6,
  "ignore_cidr_count": 0,
  "xdp": {
    "XDP_REDIRECT": {
      "rx_packets": 0,
      "rx_bytes": 0
    },
    "XDP_ABORTED": {
      "rx_packets": 0,
      "rx_bytes": 0
    },
    "XDP_PASS": {
      "rx_packets": 575936,
      "rx_bytes": 45833797
    },
    "XDP_TX": {
      "rx_packets": 0,
      "rx_bytes": 0
    },
    "XDP_DROP": {
      "rx_packets": 29172,
      "rx_bytes": 2356733
    }
  },
  "drop_tags": {
    "tags": {
      "fail2ban-sshd": {
        "rx_packets": 20922,
        "rx_bytes": 1710067
      }
    }
  },
  "ignore_tags": {
    "tags": {}
  }
}
```

## Configure Prometheus

This snippet can be used to configure Prometheus to pull the monitoring endpoint:

```yaml
scrape_configs:
  - job_name: 'couic'
    scheme: https
    authorization:
      type: Bearer
      credentials: d6ac883a-8050-4408-bf1e-5b07e9965191
    metrics_path: "/v1/metrics"
    params:
      format: ["prometheus"]
    scrape_interval: 5s
    static_configs:
      - targets: ['couic.tld:2900']
```

## Available Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `couic_drop_cidr_total` | gauge | - | Number of CIDRs in drop list |
| `couic_ignore_cidr_total` | gauge | - | Number of CIDRs in ignore list |
| `couic_stats_rx_packets_total` | counter | `action` | Packets handled by XDP per action |
| `couic_stats_rx_bytes_total` | counter | `action` | Bytes handled by XDP per action |
| `couic_drop_tag_rx_packets_total` | counter | `tag` | Packets dropped per tag |
| `couic_drop_tag_rx_bytes_total` | counter | `tag` | Bytes dropped per tag |
| `couic_ignore_tag_rx_packets_total` | counter | `tag` | Packets ignored per tag |
| `couic_ignore_tag_rx_bytes_total` | counter | `tag` | Bytes ignored per tag |
