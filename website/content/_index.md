---
title: "Couic - XDP-based Filtering with REST API Automation"
linkTitle: "Couic - XDP-based Filtering with REST API Automation"
description: "Couic is lightweight XDP-powered network filtering solution controllable through a REST API."
keywords: ["Couic", "eBPF", "XDP", "Layer 7 protection", "application layer", "DDoS mitigation", "network security", "REST API", "CTF", "FCSC"]
images: ["/images/couic-og.png"]
layout: hextra-home
---

<!-- {{< hextra/hero-badge >}}
  <div class="hx:w-2 hx:h-2 hx:rounded-full hx:bg-primary-400"></div>
  <span>75 Blocked CIDR</span>
  {{< icon name="arrow-circle-right" attributes="height=14" >}}
{{< /hextra/hero-badge >}} -->


<div class="hx:flex hx:items-center hx:mt-6 hx:mb-6">
  <svg width="60" height="60" viewBox="0 0 59 65" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;margin-right:2rem;flex-shrink:0;" aria-label="Couic logo"><path d="M32.104,0c7.064,0.013 14.17,2.349 20.08,7.139c1.62,1.313 3.078,2.751 4.372,4.291c0.251,0.298 0.495,0.6 0.733,0.905l-0.38,-0.469c2.781,3.434 2.251,8.48 -1.183,11.261c-3.389,2.745 -8.348,2.264 -11.15,-1.05c-0.07,-0.088 -0.141,-0.176 -0.213,-0.263c-0.665,-0.803 -1.419,-1.552 -2.259,-2.233c-6.866,-5.564 -16.958,-4.507 -22.522,2.36c-5.564,6.866 -4.507,16.958 2.36,22.522c6.866,5.564 16.958,4.507 22.522,-2.36c2.782,-3.433 7.828,-3.962 11.261,-1.18c3.433,2.782 3.962,7.828 1.18,11.261c-11.128,13.733 -31.312,15.847 -45.045,4.719c-13.733,-11.128 -15.847,-31.312 -4.719,-45.045c1.408,-1.738 2.962,-3.29 4.628,-4.652l-0.959,0c-1.988,0 -3.603,-1.614 -3.603,-3.603c-0,-1.988 1.614,-3.603 3.603,-3.603l21.295,0Zm20.07,15.627c-1.172,1.172 -3.074,1.172 -4.246,-0c-1.172,-1.172 -1.172,-3.074 -0,-4.246l-2.123,-2.123c-2.343,2.343 -2.343,6.149 -0,8.492c2.343,2.343 6.149,2.343 8.492,-0l-2.123,-2.123Z" fill="currentColor"/></svg>
  <div>
{{< hextra/hero-headline >}}
  eBPF firewall that cuts fast!
{{< /hextra/hero-headline >}}
  </div>
</div>

<div class="hx:mb-12">
{{< hextra/hero-subtitle >}}
  A lightweight XDP-powered network filtering solution controllable through a REST API.
{{< /hextra/hero-subtitle >}}
</div>

<div class="hx:mb-6">
{{< hextra/hero-button text="Get Started" link="docs" >}}
</div>

<div class="hx:mt-6"></div>

{{< hextra/feature-grid >}}
  {{< hextra/feature-card
    title="XDP-based Packet Processing"
    subtitle="Express Data Path (XDP) integration enables line-rate packet filtering at the network driver level, processing millions of packets per second before kernel stack ingress."
    style="background: radial-gradient(ellipse at 50% 80%,rgba(194,97,254,0.15),hsla(0,0%,100%,0));"
    icon="lightning-bolt"
  >}}
  {{< hextra/feature-card
    title="RESTful JSON API"
    subtitle="Programmable interface for dynamic rule management with atomic operations. Features role-based access control (RBAC), Unix domain socket transport, and comprehensive OpenAPI specification."
    style="background: radial-gradient(ellipse at 50% 80%,rgba(142,53,74,0.15),hsla(0,0%,100%,0));"
    icon="code"
  >}}
  {{< hextra/feature-card
    title="Static Configuration with Sets"
    subtitle="Define persistent filtering rules using .couic set files. Supports hot reloading with differential updates, adapted for scheduled tasks and infrastructure-as-code workflows."
    style="background: radial-gradient(ellipse at 50% 80%,rgba(221,210,59,0.15),hsla(0,0%,100%,0));"
    icon="collection"
  >}}
  {{< hextra/feature-card
    title="Dual-stack CIDR Support"
    subtitle="Native IPv4 and IPv6 prefix handling with automatic network address canonicalization. eBPF LPM (Longest Prefix Match) trie implementation ensures efficient lookup operations."
    icon="globe-alt"
  >}}
  {{< hextra/feature-card
    title="Automatic Entry Expiration"
    subtitle="Configurable TTL (Time-To-Live) for dynamically added entries with automatic expiration. Enables temporary mitigation strategies during active incidents without manual cleanup requirements."
    icon="clock"
  >}}
  {{< hextra/feature-card
    title="Rule Taxonomy System"
    subtitle="Tagging mechanism for rule organization and lifecycle management. Supports filtering and statistical aggregation."
    icon="tag"
  >}}
  {{< hextra/feature-card
    title="Command-line Interface"
    subtitle="The couicctl binary provides administrative operations for both local and remote management modes. Features include rule manipulation, policy control, statistics retrieval, and client authentication management."
    icon="terminal"
  >}}
  {{< hextra/feature-card
    title="Prometheus Metrics Export"
    subtitle="Native metrics endpoint exposing XDP counters, CIDR statistics, and per-tag packet/byte accounting. Enables integration with standard observability stacks for monitoring and alerting."
    icon="chart-square-bar"
  >}}
  {{< hextra/feature-card
    title="Multi-instance Synchronization"
    subtitle="Peering protocol for distributed rule propagation across infrastructure. Batched transmission with conflict resolution ensures eventual consistency in multi-node deployments."
    icon="refresh"
  >}}
{{< /hextra/feature-grid >}}
