---
title: Fail2ban
linkTitle: Fail2ban
description: "Integrate Couic with Fail2ban for automatic SSH protection. Configure Couic as Fail2ban's high-performance backend."
keywords: ["Couic", "Fail2ban", "SSH", "integration", "automatic blocking"]
images: ["/images/couic-og.png"]
prev: /docs/administration/clients-examples
next: /docs/reference
weight: 1
---

[Fail2ban](https://github.com/fail2ban/fail2ban) is a widely used solution for banning hosts that trigger multiple authentication errors. It is straightforward to configure Couic as Fail2ban’s default filtering backend, thereby benefiting from a high-performance filtering solution. This section describes the configuration files that need to be modified to achieve this integration.

## Install and configure

Fail2ban is packaged for the majority of Linux distributions. You can install it using your package manager.

```bash {filename="command"} 
sudo apt install fail2ban
```

Declare Couic as a new filtering backend by adding a new configuration in `/etc/fail2ban/action.d/couic.conf`:

```conf {filename="/etc/fail2ban/action.d/couic.conf"} 
[Definition]
# Option:  actionstart
# Notes.:  command executed on demand at the first ban (or at the start of Fail2Ban if actionstart_on_demand is set to false).
# Values:  CMD
#
actionstart =

# Option:  actionstop
# Notes.:  command executed at the stop of jail (or at the end of Fail2Ban)
# Values:  CMD
#
actionstop =

# Option:  actioncheck
# Notes.:  command executed once before each actionban command
# Values:  CMD
#
actioncheck =

# Option:  actionban
# Notes.:  command executed when banning an IP. Take care that the
#          command is executed with Fail2Ban user rights.
# Tags:    <ip>  IP address
#          <failures>  number of failures
#          <time>  unix timestamp of the ban time
# Values:  CMD
#
actionban = couicctl drop add <ip>/<cidr> -t fail2ban-<name>

# Option:  actionunban
# Notes.:  command executed when unbanning an IP. Take care that the
#          command is executed with Fail2Ban user rights.
# Tags:    <ip>  IP address
#          <failures>  number of failures
#          <time>  unix timestamp of the ban time
# Values:  CMD
#
actionunban = couicctl drop delete <ip>/<cidr>

[Init]
cidr = 32

[Init?family=inet6]
cidr = 128
```

With this configuration, Fail2ban will invoke couicctl whenever a jail is triggered.
Next, we need to configure Fail2ban to use this action for all jails. This can be done by adding the following to `/etc/fail2ban/jail.local`:

```conf {filename="/etc/fail2ban/jail.local"}  
[sshd]
backend=systemd
enabled = true

[DEFAULT]
banaction = couic
banaction_allports = couic
```

## Fail2ban and couic in action

On a system exposed to the Internet, Fail2ban should quickly begin blocking hosts that perform SSH scans, as recorded in the action logs.

```bash {filename="command"}  
couicctl drop list
```

```bash {filename="output"}  
┌────────┬──────────────────┬───────────────┬────────────┐
│ Policy ┆ CIDR             ┆ Tag           ┆ Expiration │
╞════════╪══════════════════╪═══════════════╪════════════╡
│ drop   ┆ xx.xx.xxx.xxx/32 ┆ fail2ban-sshd ┆ never      │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ drop   ┆ xx.xx.xxx.xxx/32 ┆ fail2ban-sshd ┆ never      │
└────────┴──────────────────┴───────────────┴────────────┘

```
