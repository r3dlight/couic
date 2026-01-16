#################
# Metadata Field Tests
#################

# The API supports an optional metadata field for entries.
# This test file validates metadata handling.

#################
# Create Entries with Metadata
#################

# Create entry with full metadata (kind, detail, extra)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.0.2.0/24",
  "tag": "metadata-full-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": "malicious",
    "detail": "Known botnet C2 server",
    "extra": {
      "source": "threat-intel",
      "confidence": "high",
      "first_seen": "2024-01-15"
    }
  }
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "192.0.2.0/24"
jsonpath "$.tag" == "metadata-full-test"

# Create entry with partial metadata (only kind)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "198.51.100.0/24",
  "tag": "metadata-partial-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": "suspicious"
  }
}

HTTP 400

# Create entry with empty metadata object
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "203.0.113.0/24",
  "tag": "metadata-empty-test",
  "expiration": {{expiration_ts}},
  "metadata": {}
}

HTTP 400

# Create entry with metadata containing nested extra fields
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "100.64.0.0/16",
  "tag": "metadata-nested-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": "scan",
    "detail": "Port scanning activity detected",
    "extra": {
      "ports": [22, 80, 443, 8080],
      "protocol": "tcp",
      "scan_type": "syn",
      "geo": {
        "country": "XX",
        "asn": 12345
      }
    }
  }
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "100.64.0.0/16"

# Create entry with metadata in ignore policy
POST http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.100.0.0/16",
  "tag": "metadata-ignore-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": "trusted",
    "detail": "Internal monitoring system"
  }
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "10.100.0.0/16"

#################
# Metadata with Peer Endpoint
#################

# Add entry with metadata via peer endpoint
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "add",
    "entry": {
      "cidr": "185.0.0.0/8",
      "tag": "peer-metadata-test",
      "expiration": {{expiration_ts}},
      "metadata": {
        "kind": "peer-sync",
        "detail": "Synced from peer node",
        "extra": {
          "peer_id": "node-001",
          "sync_time": 1700000000
        }
      }
    }
  }
]

HTTP 201

# Verify the entry was created
GET http://localhost/v1/drop/185.0.0.0/8
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.cidr" == "185.0.0.0/8"
jsonpath "$.tag" == "peer-metadata-test"

#################
# Metadata Edge Cases
#################

# Test metadata with null values
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.31.0.0/16",
  "tag": "metadata-null-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": null,
    "detail": "Test with null kind"
  }
}

HTTP 400

# Test metadata with very long strings
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.30.0.0/16",
  "tag": "metadata-long-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": "test",
    "detail": "This is a very long detail string that contains a lot of information about the entry. It might include descriptions, reasons for blocking, and other relevant context that helps administrators understand why this CIDR was added to the blocklist. Lorem ipsum dolor sit amet, consectetur adipiscing elit."
  }
}

HTTP 201

# Cleanup
DELETE http://localhost/v1/drop/172.30.0.0/16
Authorization: Bearer {{token}}
HTTP 204

# Test metadata with special characters in strings
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.29.0.0/16",
  "tag": "metadata-special-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": "test",
    "detail": "Contains special chars: <script>alert('xss')</script> & \"quotes\" and 'apostrophes'",
    "extra": {
      "url": "https://example.com/path?param=value&other=123"
    }
  }
}

HTTP 201

# Cleanup
DELETE http://localhost/v1/drop/172.29.0.0/16
Authorization: Bearer {{token}}
HTTP 204

# Test metadata with boolean and numeric values in extra
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.28.0.0/16",
  "tag": "metadata-types-test",
  "expiration": {{expiration_ts}},
  "metadata": {
    "kind": "test",
    "detail": "test",
    "extra": {
      "enabled": true,
      "count": 42,
      "ratio": 0.95,
      "tags": ["a", "b", "c"]
    }
  }
}

HTTP 201

# Cleanup
DELETE http://localhost/v1/drop/172.28.0.0/16
Authorization: Bearer {{token}}
HTTP 204

#################
# Cleanup
#################

DELETE http://localhost/v1/drop/192.0.2.0/24
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/drop/100.64.0.0/16
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/ignore/10.100.0.0/16
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/drop/185.0.0.0/8
Authorization: Bearer {{token}}
HTTP 204
