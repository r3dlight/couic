---
title: Static configuration using sets
linkTitle: Static Sets
description: "Configure persistent Couic firewall rules using .couic set files. Hot reload sets and import CIDRs from ASN or files."
keywords: ["Couic", "static sets", "configuration", "CIDR", "ASN import"]
images: ["/images/couic-og.png"]
prev: /docs/getting-started/dynamic-filtering
next: /docs/administration
weight: 8
---

Sets are used to manage entries for drop and ignore policies that need to persist across restarts.

## Sets overview

### Location

Sets are located in Couic working directory (see `working_dir` configuration variable).

{{< filetree/container >}}
  {{< filetree/folder name="/var/lib/couic/sets" >}}
    {{< filetree/folder name="drop" >}}
      {{< filetree/file name="bogons.couic" >}}
      {{< filetree/file name="zombies.couic" >}}
    {{< /filetree/folder >}}
    {{< filetree/folder name="ignore" >}}
      {{< filetree/file name="infra.couic" >}}
    {{< /filetree/folder >}}
  {{< /filetree/folder >}}
{{< /filetree/container >}}

### Properties

Set file example:

```bash   {filename="test.couic"}
# test set
# (comments and empty lines are ignored)

1.1.1.1/32
2606:4700:4700::1111/128
2.2.2.0/24
```

A set has several properties:

- **File format:** a text file with the `.couic` extension
- **Maximum size:** 5MB per set file
- **Name constraints:** up to 48 characters; allowed characters: `[a-zA-Z0-9-_]`
- **Scope:** node-specific (not synchronized to other nodes via peering)
- **Expiration:** entries defined in a set never expire
- **Mutability:** entries defined by a set cannot be added/removed via the API
- **Loading:** all sets are loaded at Couic startup
- **Tagging:** entries from a set are tagged with the name of the set they come from

### Using `couicctl`

```bash  {filename="command"}
couicctl drop list
```

```txt {filename="output"}
┌────────┬──────────────────────────┬────────────┬────────────┐
│ Policy ┆ CIDR                     ┆ Tag        ┆ Expiration │
╞════════╪══════════════════════════╪════════════╪════════════╡
│ drop   ┆ 8.8.8.8/32               ┆            ┆ never      │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ drop   ┆ 8.8.8.8/24               ┆            ┆ never      │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ drop   ┆ 1.0.0.1/32               ┆ test.couic ┆ never      │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ drop   ┆ 2.2.2.2/24               ┆ test       ┆ never      │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ drop   ┆ 1.1.1.1/32               ┆ test.couic ┆ never      │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ drop   ┆ 2606:4700:4700::1111/128 ┆ test.couic ┆ never      │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ drop   ┆ 2606:4700:4700::1001/128 ┆ test.couic ┆ never      │
└────────┴──────────────────────────┴────────────┴────────────┘
```

You can hot reload the sets on a node using the API or CLI with the command: `couicctl sets reload`. This command performs a differential update between the current entries in memory and the set files, ensuring that existing blocks remain unchanged if they are not modified.

{{< callout type="info" >}}
Hot reloading the sets allows for easy integration of Couic into scheduled tasks like crontab
{{< /callout >}}

## Creating Sets

### Add sets manually

You can create persistent file-based sets by manually creating `.couic` files in `/var/lib/couic/sets/{drop,ignore}/`. These are automatically loaded at Couic startup and can be hot-reloaded with `couicctl sets reload`.

{{< callout type="warning" >}}
Set files must be owned by the `couic` user and have `600` permissions (read/write for owner only) for security reasons. Couic will refuse to load files with incorrect ownership or permissions.
{{< /callout >}}

#### Add local network to ignore policy

```bash   {filename="command"}
sudo touch /var/lib/couic/sets/ignore/lan.couic
sudo chown couic: /var/lib/couic/sets/ignore/lan.couic
sudo chmod 600 /var/lib/couic/sets/ignore/lan.couic
sudo vim /var/lib/couic/sets/ignore/lan.couic
sudo couicctl sets reload
```

```txt {filename="output"}
Sets reloaded successfully
```

#### List ignore policy entries:

```bash   {filename="command"}
couicctl ignore list
```

```txt {filename="output"}
┌────────┬────────────────┬───────────┬────────────┐
│ Policy ┆ CIDR           ┆ Tag       ┆ Expiration │
╞════════╪════════════════╪═══════════╪════════════╡
│ ignore ┆ 192.168.0.0/24 ┆ lan.couic ┆ never      │
└────────┴────────────────┴───────────┴────────────┘
```

### Add sets using `couicctl`

#### Block an entire ASN using RIPE database

The `couicctl sets create` command supports importing prefixes directly from an ASN via the RIPE NCC RIPEstat API:

```bash   {filename="command"}
couicctl sets create --from-asn AS200373 drop asn200373
```

```txt {filename="output"}
Fetching prefixes for ASN: AS200373
Retrieved 42 prefixes from RIPE NCC RIPEstat
Set created successfully with 42 entries
Don't forget to run 'couicctl sets reload' to apply the changes
```

{{< callout type="info" >}}
The ASN can be specified with or without the "AS" prefix: both `AS200373` and `200373` are valid.
{{< /callout >}}

To apply the changes:

```bash   {filename="command"}
couicctl sets reload
```

#### Import CIDRs from a file

You can bulk-import CIDRs from a text file using the `--from-file` option:

```bash   {filename="file.txt"}
# Malicious IPs from threat intel feed
203.0.113.0/24
198.51.100.0/24
# IPv6 ranges
2001:db8::/32

# Empty lines and comments are ignored
192.0.2.0/24
```

```bash   {filename="command"}
couicctl sets create --from-file file.txt drop threat-intel
```

```txt {filename="output"}
Reading CIDRs from file: file.txt
Loaded 4 CIDRs from file
Set created successfully with 4 entries
Don't forget to run 'couicctl sets reload' to apply the changes
```

#### Bulk operations with xargs

You can combine the new helpers with `xargs` for bulk operations:

##### Batch import with reload at the end

```bash   {filename="command"}
cat cidr-list.txt | xargs couicctl sets create drop blocklist && couicctl sets reload
```