---
linkTitle: "Documentation"
title: Introduction
description: "Couic documentation — Learn how to deploy and configure an XDP-based defense system with REST API automation"
keywords: ["Couic", "eBPF", "XDP", "Layer 7 DDoS protection", "application layer firewall", "Linux network security", "REST API firewall"]
images: ["/images/couic-og.png"]
next: /docs/get-couic
weight: 1
---

Welcome to the Couic documentation!


<!--more-->

## What is Couic?

**Couic** `[kwɪk]` is a lightweight eBPF-powered network filtering solution specifically designed to defend against **Layer 7 (application layer) DDoS attacks**.
It attaches on network interfaces using [XDP](https://en.wikipedia.org/wiki/Express_Data_Path) and then exposes an HTTP REST API to manage blocklists and allowlists.

**Couic** has been used in production since 2021, evolving every year and taking different forms to adapt to the needs of the [France Cybersecurity Challenge](https://fcsc.fr/)🦕 infrastructure. This [CTF](https://en.wikipedia.org/wiki/Capture_the_flag_(cybersecurity)) competition, organized every year by [ANSSI](https://cyber.gouv.fr/offre-de-service/formations-entrainement-et-decouverte-des-metiers/challenges/france-cybersecurity-challenge/) (the French national cybersecurity agency), requires a good level of protection against platform overload caused by CTF participants as well as DDoS attacks, especially at layer 7. 

This program is designed to complement the L3/L4 protection measures implemented by the hosting provider by focusing on **application-layer threats** that bypass lower-layer defenses. It aims to be as simple and efficient as possible and to work alongside existing filtering solutions on the server (iptables, ipsets, nftables...).

**Couic** was presented for the first time during SSTIC2024 symposium[^1].

## Features

- Linux 5.11+ support
- **IPv4** and **IPv6** CIDRs support
- **Static configuration** using set files
- **Dynamic configuration** with the JSON API
- **Ease of use** with the provided CLI
- **Anti Lock-out system** thanks to IGNORE and DROP filtering policies
- **Automatic expiration** of API-added entries
- **Tagging** to facilitate entry management
- **Real-time monitoring** of network and eBPF-program performance with Prometheus exporter endpoint
- **Reporting** with webhook notifications for filtering activity
- **Simple Synchronisation** between distributed instances of Couic
- **High performance** packet processing

## Project Architecture

The project provides a programmable firewall built around two core components, with auxiliary tools provided as integration examples:

![Couic System Architecture](/images/couic-architecture.svg "Couic System Architecture")

| Component | Type | Purpose | Status |
|-----------|------|---------|--------|
| `couic` | Core | eBPF filtering daemon with REST API (via `Unix Domain Socket`) | Public |
| `couicctl` | Core | Command-line administration tool | Public |
| `couic-report` | Auxiliary | Telemetry aggregation and notifications (Discord) | Public |
| `couicmon` | Auxiliary | Log-based automatic rule injection | Coming soon |
| `client` | Library | REST API client implementation | Public |
| `common` | Library | Shared types and definitions for Couic components | Public |

{{< callout >}}
Couic is designed as a filtering backend. Users are encouraged to connect their own solutions or develop custom modules using the REST API. An [OpenAPI specification](/json/openapi.json) is provided to facilitate client development.
{{< /callout >}}

[^1]:
    Read more about Couic story (previously Hodor) in the paper (french): [SSTIC2024 - Retour d’expérience sur l’organisation d’un CTF :
    Rétrospective de 5 ans de FCSC](https://www.sstic.org/media/SSTIC2024/SSTIC-actes/ctf_fcsc/SSTIC2024-Article-ctf_fcsc-thuau_iooss_court_jean_olivier_claverie.pdf)

