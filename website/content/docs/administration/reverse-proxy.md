---
title: Reverse Proxy
linkTitle: Reverse Proxy
description: "Expose Couic API behind a reverse proxy with Nginx. Enable secure remote access with TLS and access controls."
keywords: ["Couic", "reverse proxy", "Nginx", "TLS", "remote access"]
images: ["/images/couic-og.png"]
prev: /docs/administration/auth
next: /docs/administration/monitoring
weight: 2
---

Exposing Couic’s API behind a reverse proxy can be beneficial in several scenarios, such as:

* enabling secure remote management of filtering rules,
* providing controlled access to the monitoring endpoint,
* facilitating peering between multiple Couic instances.

{{< callout type="error" >}}
It is the administrator’s responsibility to ensure that the reverse proxy is configured in accordance with security best practices. This includes enforcing TLS for confidentiality, applying strict filtering policies to limit exposure, and maintaining proper access controls to protect the integrity of communications.
{{< /callout >}}

## Example using nginx

In order for Nginx to access Couic’s socket `/var/run/couic/couic.sock`, the Nginx user must be added to the `couic` group:

```bash {filename="command"}
sudo usermod -aG couic www-data
```

Nginx must be restarted for the change to take effect.

```bash {filename="command"}
sudo systemctl restart nginx
```

Below is an example Nginx configuration that exposes Couic’s API on port 2900 (TLS) and restricts access to only two IP addresses.

```bash {filename="default.conf"}
# Default server
server {
  listen 0.0.0.0:80 default_server;
  server_name _;

  # Redirect to HTTPS
  location / {
    return 301 https://$host$request_uri;
  }
}

# Upstream configuration
upstream couic {
  server unix:/var/run/couic/couic.sock;
}

# Couic
server {
  server_name couic.tld;
  listen x.x.x.x:2900 ssl;

  ssl_certificate /etc/acme/certs/couic.tld_ecc/fullchain.cer;
  ssl_certificate_key /etc/acme/certs/couic.tld_ecc/couic.tld.key;

  ssl_protocols TLSv1.3;
  ssl_prefer_server_ciphers off;

  # ACL
  allow x.x.x.x;
  allow x.x.x.x;
  deny all;

  location / {
    proxy_set_header X-Forwarded-Host $host;
    proxy_set_header X-Forwarded-Server $host;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_pass http://couic;
    proxy_http_version 1.1;
    proxy_pass_request_headers on;
    proxy_set_header Connection "keep-alive";
    proxy_store off;
  }
}
```

## Test reverse proxying

### Using curl

Test using curl and `couicctl` token:

```bash {filename="command"}
curl https://couic.tld:2900/v1/stats \
      -H "Accept: application/json"  \
      -H "Authorization: Bearer 79deb94f-5dd1-417f-8842-667d8dff4480"
```

```json {filename="output"}
{
  "drop_cidr_count": 8,
  "ignore_cidr_count": 0,
  "xdp": {
    "XDP_ABORTED": {
      "rx_packets": 0,
      "rx_bytes": 0
    },
    "XDP_DROP": {
      "rx_packets": 29151,
      "rx_bytes": 2355239
    },
    "XDP_TX": {
      "rx_packets": 0,
      "rx_bytes": 0
    },
    "XDP_REDIRECT": {
      "rx_packets": 0,
      "rx_bytes": 0
    },
    "XDP_PASS": {
      "rx_packets": 574537,
      "rx_bytes": 45724001
    }
  }
}
```

### Using `couicctl` in remote mode

You can also test the remote API using the cli in remote mode. Edit the `couicctl` configuration file:

```toml {filename="/etc/couic/couicctl.toml"}
#==========================
# Couicctl Configuration File
#==========================

# mode: local or remote
mode = "remote"

# Local server configuration
#socket = "/var/run/couic/couic.sock"
# Auth token
#client_file = "/var/lib/couic/rbac/clients/couicctl.toml"

# Remote server configuration
tls = true
host = "couic.tld"
port = 2900
token = "00000000-0000-0000-0000-000000000000"
# get remove token using: couicctl clients list on remote server
```
