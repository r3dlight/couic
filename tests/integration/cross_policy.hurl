#################
# Cross-Policy Tests
#################

# Tests for behavior when the same CIDR exists in both DROP and IGNORE policies

#################
# Same CIDR in Both Policies
#################

# Create entry in drop policy
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.50.0/24",
  "tag": "cross-policy-drop",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "192.168.50.0/24"
jsonpath "$.tag" == "cross-policy-drop"

# Create same CIDR in ignore policy (should this be allowed?)
POST http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.50.0/24",
  "tag": "cross-policy-ignore",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "192.168.50.0/24"
jsonpath "$.tag" == "cross-policy-ignore"

# Verify both entries exist independently
GET http://localhost/v1/drop/192.168.50.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" == "cross-policy-drop"

GET http://localhost/v1/ignore/192.168.50.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" == "cross-policy-ignore"

# Delete from drop should not affect ignore
DELETE http://localhost/v1/drop/192.168.50.0/24
Authorization: Bearer {{token}}

HTTP 204

# Verify ignore entry still exists
GET http://localhost/v1/ignore/192.168.50.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" == "cross-policy-ignore"

# Verify drop entry is gone
GET http://localhost/v1/drop/192.168.50.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404

# Cleanup ignore entry
DELETE http://localhost/v1/ignore/192.168.50.0/24
Authorization: Bearer {{token}}

HTTP 204

#################
# IPv6 Cross-Policy
#################

# Create IPv6 entry in drop
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "2001:db8:1::/48",
  "tag": "ipv6-cross-drop",
  "expiration": {{expiration_ts}}
}

HTTP 201

# Create same IPv6 CIDR in ignore
POST http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "2001:db8:1::/48",
  "tag": "ipv6-cross-ignore",
  "expiration": {{expiration_ts}}
}

HTTP 201

# Verify both exist
GET http://localhost/v1/drop/2001:db8:1::/48
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.tag" == "ipv6-cross-drop"

GET http://localhost/v1/ignore/2001:db8:1::/48
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.tag" == "ipv6-cross-ignore"

# Cleanup
DELETE http://localhost/v1/drop/2001:db8:1::/48
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/ignore/2001:db8:1::/48
Authorization: Bearer {{token}}
HTTP 204

#################
# Peer Sync Cross-Policy
#################

# Add entries via peer to both policies
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "add",
    "entry": {
      "cidr": "10.50.0.0/16",
      "tag": "peer-drop",
      "expiration": {{expiration_ts}}
    }
  }
]

HTTP 201

POST http://localhost/v1/ignore/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "add",
    "entry": {
      "cidr": "10.50.0.0/16",
      "tag": "peer-ignore",
      "expiration": {{expiration_ts}}
    }
  }
]

HTTP 201

# Verify both policies have the entry
GET http://localhost/v1/drop
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$[*].cidr" contains "10.50.0.0/16"

GET http://localhost/v1/ignore
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$[*].cidr" contains "10.50.0.0/16"

# Remove from one policy via peer
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "remove",
    "entry": {
      "cidr": "10.50.0.0/16",
      "expiration": 0
    }
  }
]

HTTP 201

# Verify drop is gone but ignore remains
GET http://localhost/v1/drop/10.50.0.0/16
Authorization: Bearer {{token}}

HTTP 404

GET http://localhost/v1/ignore/10.50.0.0/16
Authorization: Bearer {{token}}

HTTP 200

# Cleanup
DELETE http://localhost/v1/ignore/10.50.0.0/16
Authorization: Bearer {{token}}
HTTP 204

#################
# List Operations with Cross-Policy Entries
#################

# Create multiple entries in both policies
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.16.1.0/24",
  "tag": "list-test-1",
  "expiration": {{expiration_ts}}
}

HTTP 201

POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.16.2.0/24",
  "tag": "list-test-2",
  "expiration": {{expiration_ts}}
}

HTTP 201

POST http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.16.1.0/24",
  "tag": "list-test-ignore-1",
  "expiration": {{expiration_ts}}
}

HTTP 201

POST http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.16.3.0/24",
  "tag": "list-test-ignore-3",
  "expiration": {{expiration_ts}}
}

HTTP 201

# List drop entries - should only show drop entries
GET http://localhost/v1/drop
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$[*].cidr" contains "172.16.1.0/24"
jsonpath "$[*].cidr" contains "172.16.2.0/24"
jsonpath "$[*].tag" not contains "list-test-ignore-1"
jsonpath "$[*].tag" not contains "list-test-ignore-3"

# List ignore entries - should only show ignore entries
GET http://localhost/v1/ignore
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$[*].cidr" contains "172.16.1.0/24"
jsonpath "$[*].cidr" contains "172.16.3.0/24"
jsonpath "$[*].tag" not contains "list-test-1"
jsonpath "$[*].tag" not contains "list-test-2"

# Cleanup
DELETE http://localhost/v1/drop/172.16.1.0/24
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/drop/172.16.2.0/24
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/ignore/172.16.1.0/24
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/ignore/172.16.3.0/24
Authorization: Bearer {{token}}
HTTP 204

#################
# Stats Verification
#################

# Create entries for stats test
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.0.2.128/25",
  "tag": "stats-drop-test",
  "expiration": {{expiration_ts}}
}

HTTP 201

POST http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "198.51.100.128/25",
  "tag": "stats-ignore-test",
  "expiration": {{expiration_ts}}
}

HTTP 201

# Verify stats show correct counts
GET http://localhost/v1/stats
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.drop_cidr_count" >= 1
jsonpath "$.ignore_cidr_count" >= 1

# Verify metrics show both policies
GET http://localhost/v1/metrics
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$.drop_cidr_count" >= 1
jsonpath "$.ignore_cidr_count" >= 1
jsonpath "$.drop_tags" isCollection
jsonpath "$.ignore_tags" isCollection

# Cleanup
DELETE http://localhost/v1/drop/192.0.2.128/25
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/ignore/198.51.100.128/25
Authorization: Bearer {{token}}
HTTP 204
