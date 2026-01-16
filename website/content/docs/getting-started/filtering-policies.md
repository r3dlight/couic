---
title: Filtering Policies
linkTitle: Filtering Policies
description: "Understand Couic drop and ignore filtering policies. Learn how XDP packet matching works with eBPF LPM tries."
keywords: ["Couic", "filtering policies", "drop", "ignore", "XDP", "LPM trie"]
images: ["/images/couic-og.png"]
prev: /docs/getting-started
next: /docs/getting-started/dynamic-filtering
weight: 6
---

## General

Couic applies the following filtering on the machine's network traffic:

- **Only incoming traffic is filtered** (see below)
- Filtering applies at the packet level, before the kernel network stack processes the traffic.
- A single entry is a [CIDR](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
- To keep the XDP program as simple as possible, **filtering applies to all ports**
- Two filtering policies are managed: `drop` and `ignore`
- **Rules added via the API are volatile** and do not persist across restarts
- **Persistent rules** can be configured using [static sets](/docs/getting-started/static-sets)

{{< callout type="info" >}}
XDP operates only on incoming traffic. It hooks packets immediately after they arrive from the network interface, before they reach the kernel networking stack. Outgoing packets cannot be processed by XDP because egress occurs after routing and socket handling.

```ascii
NIC ---> [XDP hook] ---> Kernel network stack ---> Egress
            ^
            |
       Ingress only
```
See Wikipedia: [Packet flow paths in the Linux kernel](https://en.wikipedia.org/wiki/Express_Data_Path#/media/File:Netfilter-packet-flow.svg)
{{< /callout >}}

## `drop` and `ignore` policies

The `ignore` filtering policy allows adding subnets that will never be blocked. This target always takes precedence over the `drop` target. This means that if the same subnet is added to both tables, traffic from that subnet will be allowed. In the context of automatic rule injection via the API, this feature allows administrators to prevent unwanted access interruptions.

```bash
      Incoming packet
            |
            v
  Is CIDR in IGNORE Policy?
            |
  +---------+---------+
  |                   |
  Yes                  No
  |                   |
[ALLOW]               v
            Is CIDR in DROP Policy?
                      |
                +-----+-----+
                |           |
                Yes          No
                |           |
              [DROP]      [ALLOW]
```

{{< callout type="important" >}}
It is highly recommended to add all critical infrastructure IPs, such as DNS, NTP, gateways, and administration IPs (e.g., SSH), to the `ignore` target. This ensures uninterrupted access to essential services and administrative functions.
{{< /callout >}}

## Rules storage

### Architecture

Couic uses a **two-layer data structure** for efficient rule management and packet filtering:

1. **User-Space Management Layer**  
   Each rule category (`drop_v4`, `drop_v6`, `ignore_v4`, `ignore_v6`) maintains a HashMap in user space that stores rule metadata such as CIDR addresses, creation timestamps, expiration times, and associated tags. This layer handles rule additions, removals, and automatic cleanup of expired entries.

2. **Kernel-Space Filtering Layer**  
   Each category has a corresponding **eBPF Longest Prefix Match (LPM) trie** in kernel space for ultra-fast packet matching. IPv4 and IPv6 rules are stored separately in optimized data structures that enable efficient longest-prefix matching directly in the kernel, avoiding costly user-space lookups for each packet.

This design combines the flexibility of user-space management with the high performance of kernel-space filtering.

{{< callout type="info" >}}
For more details on how [eBPF Longest Prefix Match (LPM)](https://docs.kernel.org/bpf/map_lpm_trie.html) tries work, refer to the well-documented [`BPF_MAP_TYPE_LPM_TRIE` source code](https://elixir.bootlin.com/linux/v6.16.12/source/kernel/bpf/lpm_trie.c) in the Linux kernel.
{{< /callout >}}

## Network addresses normalization

When storing rules in the Couic store, the system is designed to **normalizes all CIDR entries to their network address**.

For example:

- If a rule is added for 192.168.0.1/24, the store calculates the network address as 192.168.0.0/24 and stores this value.
- If a subsequent rule is added for 192.168.0.2/24, the store again calculates the network address as 192.168.0.0/24. Since this network is already present, the store recognizes it as a duplicate or overlapping entry.
- **Result**: Only one entry for 192.168.0.0/24 will exist in the store, regardless of which IP within the /24 subnet was originally specified.

Rule storage examples:

| Input CIDR            | Stored Network Address |
|-----------------------|:----------------------:|
| 192.168.1.0/32        | 192.168.1.0/32         |
| 192.168.1.1/32        | 192.168.1.1/32         |
| 192.168.1.2/24        | 192.168.1.0/24         |
| 192.168.1.200/24      | 192.168.1.0/24         |

This approach ensures consistency, eliminates redundancy, and aligns with the Linux kernel’s internal handling of prefix-based matching.

## How packets are matched

When a packet arrives:

1. The **eBPF LPM trie** performs a prefix match on the packet’s source or destination IP address.
2. The trie returns the rule corresponding to the **longest matching prefix**.
3. That rule is applied (drop, ignore, etc.), and processing continues.

### Matching with Overlapping Prefixes

Multiple entries can exist that cover overlapping address ranges, as long as their **prefix lengths differ**. When a packet is processed by the xdp program, the underlying linux LPM Trie always returns the **most specific (longest) matching prefix**.

Suppose the store contains two entries:

- `192.168.0.0/24` (covers `192.168.0.0 – 192.168.0.255`)  
- `192.168.0.0/25` (covers `192.168.0.0 – 192.168.0.127`)  

Now, lookups behave as follows:

| Input Address   | Matching Entry   | Why? |
|-----------------|------------------|------|
| `192.168.0.10`  | `192.168.0.0/25` | Falls into both `/24` and `/25`, but `/25` is longer → more specific. |
| `192.168.0.200` | `192.168.0.0/24` | Falls into `/24`, but not into `/25`. |
| `10.0.0.1`      | No match         | Does not fall into any prefix. |

## Performance

Couic leverages XDP (eXpress Data Path) to achieve near line-rate packet processing. Depending on hardware capabilities and NIC driver support, XDP can filter **millions of packets per second** per core, with minimal CPU overhead since packets are processed before entering the kernel network stack.

The Couic API is designed for high-throughput rule management, capable of handling **tens to hundreds of thousands of rule insertions per second** depending on hardware specifications. This ensures responsive filtering even during large-scale DDoS attacks requiring rapid rule propagation.

## Limitations

The default size of the underlying Ebpf maps is set as follows:

| Max Records | Type    | Policy    |
|-------------|---------|-----------|
| 262 144     | IPv4    | `drop`    |
| 262 144     | IPv6    | `drop`    |
| 65 536      | IPv4    | `ignore`  |
| 65 536      | IPv6    | `ignore`  |

{{< callout type="info" >}}
These values are defined by `MAX_DROP_ENTRIES` and `MAX_IGNORE_ENTRIES` constants and can be modified at compile time when building the eBPF program.
{{< /callout >}}