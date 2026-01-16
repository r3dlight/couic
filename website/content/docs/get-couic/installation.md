---
title: Installation
linkTitle: Installation
description: "Install Couic on Linux — Download pre-compiled Debian (.deb) or RPM packages, configure systemd service, and verify deployment"
keywords: ["Couic", "installation", "Debian", "Ubuntu", "RPM", "Fedora", "systemd", "Linux 5.11"]
images: ["/images/couic-og.png"]
prev: /docs/get-couic/
next: /docs/get-couic/build-from-source
weight: 3
---

## Pre-compiled Binaries and Packages

Pre-compiled binaries, Debian (`.deb`) and RPM (`.rpm`) packages are available for download in the [Releases](https://github.com/FCSC-FR/couic/releases) section of the project on Github.

{{< callout type="info" >}}
For portability, the distributed binaries are statically compiled using [musl](https://musl.libc.org/).
{{< /callout >}}

The easiest way to deploy Couic is to use the pre-compiled packages. The following sections use the Debian package as an example.

## Installing the Debian package

{{% steps %}}

### Install Debian package

```bash {filename="command"}
sudo dpkg -i couic_1.0.0-1_amd64.deb
```

### Edit Couic configuration

The service needs to be configured before you can enable and start the systemd service.

```toml {filename="/etc/couic/couic.toml"}
#==========================
# Couic Configuration File
#==========================

ifaces = ["ens3"]
working_dir = "/var/lib/couic"
user = "couic"
group = "couic"

[logging]
dir = "/var/log/couic"

[server]
socket = "/var/run/couic/couic.sock"
```

The `ifaces` variable must be edited to match your environment. At the startup of Couic, the eBPF/XDP module will be attached to the specified interface(s).

{{< callout type="info" >}}
Depending on the hardware configuration, the XDP program can be loaded in the following operation modes: Native, Offloaded, Generic. For more information, refer to the [Cilium project documentation](https://docs.cilium.io/en/stable/reference-guides/bpf/progtypes/) (XDP operation modes and Driver support sections).
{{< /callout >}}

{{< callout type="important" >}}
By default, Couic attaches the XDP program in **Generic mode** to ensure broad compatibility across diverse hardware and driver configurations. An undocumented `operation_mode` configuration option exists in the configuration file, supporting `generic`, `native`, and `offloaded` modes. However, this feature is still experimental and requires further testing before being officially supported.
{{< /callout >}}

### Enable and start Systemd service

```bash {filename="command"}
sudo systemctl enable couic
sudo systemctl start couic
```

### Verify Couic installation

You can verify couic is operational using `systemctl`.

```bash {filename="command"}
sudo systemctl status couic
```

```txt {filename="output"}
● couic.service - couic
     Loaded: loaded (/usr/lib/systemd/system/couic.service; enabled; preset: enabled)
     Active: active (running) since Thu 2026-01-15 09:32:25 UTC; 6s ago
 Invocation: 098f5f47d05c40868e318530da1ffb90
   Main PID: 33190 (couic)
      Tasks: 14 (limit: 2281)
     Memory: 12.4M (peak: 34.7M)
        CPU: 82ms
     CGroup: /system.slice/couic.service
             └─33190 /usr/sbin/couic -c /etc/couic/couic.toml

Jan 15 09:32:25 couic-01 systemd[1]: Starting couic.service - couic...
Jan 15 09:32:25 couic-01 systemd[1]: Started couic.service - couic.
Jan 15 09:32:25 couic-01 couic[33190]: Starting Couic version 1.0.0
Jan 15 09:32:25 couic-01 couic[33190]: 2026-01-15T09:32:25.251921Z  INFO couic::firewall::service: XDP program attached to interface: ens3 (mode: Generic)
Jan 15 09:32:25 couic-01 couic[33190]: 2026-01-15T09:32:25.252551Z  INFO couic::firewall::service: sets reload: policy=ignore, updated=0, removed=0, created=2
Jan 15 09:32:25 couic-01 couic[33190]: 2026-01-15T09:32:25.252612Z  INFO couic::firewall::service: sets reload: policy=drop, updated=0, removed=0, created=0
```

{{% /steps %}}
