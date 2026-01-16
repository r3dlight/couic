---
title: Security and Privilege Model
linkTitle: Security and Privilege Model
description: "Couic security design — Memory-safe Rust implementation, privilege dropping with seccomp, and defense-in-depth hardening"
keywords: ["Couic", "security", "privilege dropping", "seccomp", "Rust memory safety", "eBPF security", "hardened binaries"]
images: ["/images/couic-og.png"]
prev: /docs/reference/project-architecture
next: /docs/reference/couicctl
weight: 2
---

Couic deploys an **eBPF program** into the Linux kernel and attaches an **XDP hook** to one or more network interfaces. By nature, these operations require elevated privileges. To minimize its attack surface, Couic implements several defense-in-depth techniques.

## Memory Safety

Couic is implemented in **Rust**, a programming language designed with safety and concurrency in mind. Rust's ownership model and strict compile-time checks eliminate entire classes of memory safety issues, such as buffer overflows, null pointer dereferences, and use-after-free errors. These properties make Rust an good option for building a filtering software.

Couic adheres to Rust's philosophy of minimizing the use of `unsafe` code. The only exceptions are in the eBPF module, where `unsafe` is required to interact with low-level kernel APIs and data structures. Even in these cases, the use of `unsafe` is kept to the absolute minimum and is carefully reviewed to ensure correctness and safety. This approach balances the need for low-level control with Rust's strong safety guarantees.

## Privilege Management

Couic is designed to run as a **non-privileged user**. At startup, the process requires the `CAP_SYS_ADMIN` and `CAP_NET_ADMIN` Linux capabilities to load its eBPF program and attach the XDP hook to network interfaces. Immediately after completing these operations, Couic **drops all capabilities**, ensuring it operates with the lowest possible privilege level during runtime.

By default, installation creates a dedicated system user, `couic`. All runtime resources and artifacts are owned and managed by this user. Couic enforces strict file permission checks on sensitive resources and will refuse to start if security conditions are not met.

## Systemd Sandboxing

The Debian and rpm packages install a systemd unit configured with **sandboxing features**. These restrictions protect the filesystem, constrain available system calls, and limit the overall attack surface of the Couic service process. This ensures that even if compromised, the service has minimal ability to affect the host environment.

{{< callout type="info" >}}
For users seeking to further harden their systemd unit configuration, [Systemd Hardening Helper (shh)](https://github.com/synacktiv/shh) is a valuable resource. This tool automates the analysis of running services and generates tailored hardening recommendations based on observed runtime behavior.
{{< /callout >}}

## Kernel and User Space Separation

The **XDP program** implemented by Couic is intentionally kept as simple as possible. All complex logic is executed in **user space**, while the kernel component is limited to efficient packet processing. In practice, Couic maintains rule sets and metadata in user-space hash maps, while only **add** and **delete** operations are synchronized with eBPF maps in the kernel. This design provides both performance and maintainability, while reducing the risk of kernel-space complexity.

## Hardened Binaries

All binaries produced by the project are built with **hardened compiler options** by default, including PIE, symbol stripping and RELRO. These mitigations raise the bar against common exploitation techniques.

```bash {filename="command"} 
checksec --file=release/couic
```

```bash {filename="output"} 
RELRO           STACK CANARY      NX            PIE             RPATH      RUNPATH      Symbols         FORTIFY Fortified       Fortifiable     FILE
Full RELRO      No canary found   NX enabled    PIE enabled     No RPATH   No RUNPATH   No Symbols        No    0               0               release/couic
```

## Communication Model

By default, the Couic HTTP server listens on a **Unix domain socket** rather than a TCP socket. While more restrictive to configure, this approach allows fine-grained control over access permissions and provides significantly better performance for local API requests.

## API Authentication

All access to Couic’s API is authenticated through a **lightweight role-based access control (RBAC) mechanism**, described in detail in the [Authentication and Authorization](/docs/administration/auth.html) section of this documentation.
