---
title: Peering
linkTitle: Peering
description: "Synchronize Couic rules across nodes — Configure peer-to-peer replication for distributed firewall deployments with batched propagation"
keywords: ["Couic", "peering", "synchronization", "distributed firewall", "cluster", "rule replication", "multi-node"]
images: ["/images/couic-og.png"]
prev: /docs/administration/monitoring
next: /docs/administration/reporting
weight: 4
---

Couic provides a simple mechanism for synchronizing dynamic filtering rules between distributed instances of Couic. This peering mechanism is suitable for small deployments. For larger deployments, it is recommended to use more robust methods, such as a message bus like Kafka.

Couic internally mitigates peer load within a cluster by batching the transmission of rule insertions and deletions at fixed **250 ms intervals**. This controlled update mechanism ensures efficient synchronization without overwhelming peers. The design is especially critical during DDoS attacks, when the system may need to process and propagate several thousand rules per second while maintaining cluster stability and responsiveness.

{{< callout type="info" >}}
This section assumes that remote access to the Couic API has already been set up using a reverse proxy.
For instructions on enabling remote access, see the [Reverse Proxy section](/docs/administration/reverse-proxy) first.
{{< /callout >}}

## Configuration

This section illustrates the configuration required for a cluster consisting of two instances running Couic.

### Add a peering client

Add a new client to `peering` group on each instance composing the cluster:

```bash {filename="command@couic1"}
couicctl clients add -n couic2 -g peering
```

```bash {filename="output"}
┌─────────────┬────────────┬──────────────────────────────────────┐
│ Name        ┆ Group      ┆ Token                                │
╞═════════════╪════════════╪══════════════════════════════════════╡
│ couic2      ┆ peering    ┆ e78336b3-8128-4c84-88c7-e2fad9c32d99 │
└─────────────┴────────────┴──────────────────────────────────────┘
```

```bash {filename="command@couic2"}
couicctl clients add -n couic1 -g peering
```

```bash {filename="output"}
┌─────────────┬────────────┬──────────────────────────────────────┐
│ Name        ┆ Group      ┆ Token                                │
╞═════════════╪════════════╪══════════════════════════════════════╡
│ couic1      ┆ peering    ┆ bbfa1388-3218-463c-9722-6805507c14bb │
└─────────────┴────────────┴──────────────────────────────────────┘
```

{{< callout type="info" >}}
For more details about client permissions see [Authentication and Authorization](/docs/administration/auth).
{{< /callout >}}

### Configure Couic to enable peering

Add the peering configuration to `couic1` instance.

```toml {filename="/etc/couic/couic.toml"}
[peering]
enabled = true

[[peering.peers]]
host = "couic2.couic.tld"
port = 2900
tls = true
token = "bbfa1388-3218-463c-9722-6805507c14bb"
```

Add the peering configuration to `couic2` instance.

```toml {filename="/etc/couic/couic.toml"}
[peering]
enabled = true

[[peering.peers]]
host = "couic1.couic.tld"
port = 2900
tls = true
token = "e78336b3-8128-4c84-88c7-e2fad9c32d99"
```

Restart couic on each instances to load the new configuration.

```bash {filename="command"}
sudo systemctl restart couic
```

## Test using `couicctl`

Use `couicctl` to drop incoming traffic for a single IP on `couic1` instance:

```bash {filename="command@couic1"}
couicctl drop add 1.1.1.1/32 -e 1m
```

```bash {filename="output"}
┌────────┬────────────┬─────┬────────────┐
│ Policy ┆ CIDR       ┆ Tag ┆ Expiration │
╞════════╪════════════╪═════╪════════════╡
│ drop   ┆ 1.1.1.1/32 ┆     ┆ 59s        │
└────────┴────────────┴─────┴────────────┘
```

The previous rule should now be automatically synchronized to the `couic2` instance:

```bash {filename="command@couic2"}
couicctl drop list
```

```bash {filename="output"}
┌────────┬────────────┬─────┬────────────┐
│ Policy ┆ CIDR       ┆ Tag ┆ Expiration │
╞════════╪════════════╪═════╪════════════╡
│ drop   ┆ 1.1.1.1/32 ┆     ┆ 55s        │
└────────┴────────────┴─────┴────────────┘
```
