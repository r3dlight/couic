#################
# Drop Policy API Tests
#################

# List all drop entries (should be empty initially)
GET http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" isCollection

#################
# Create Drop Entries
#################

# Create a new IPv4 drop entry - Success
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": "test-drop-ipv4",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "192.168.1.0/24"
jsonpath "$.tag" == "test-drop-ipv4"
jsonpath "$.expiration" == {{expiration_ts}}

# Create a new IPv6 drop entry - Success
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "2001:db8::/32",
  "tag": "test-drop-ipv6",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "2001:db8::/32"
jsonpath "$.tag" == "test-drop-ipv6"
jsonpath "$.expiration" == {{expiration_ts}}

# Create drop entry without tag (should work)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.0.0.0/8",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "10.0.0.0/8"
jsonpath "$.expiration" == {{expiration_ts}}

# Try to create duplicate drop entry - Should fail
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": "duplicate-test",
  "expiration": {{expiration_ts}}
}

HTTP 409

# Try to create entry with invalid CIDR - Should fail
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "invalid-cidr",
  "tag": "invalid-test",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Try to create entry with invalid expiration - Should fail
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.16.0.0/16",
  "tag": "invalid-expiration",
  "expiration": -1
}

HTTP 400

#################
# List Drop Entries (after creation)
#################

GET http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$" count == 3
jsonpath "$[*].cidr" contains "192.168.1.0/24"
jsonpath "$[*].cidr" contains "2001:db8::/32"
jsonpath "$[*].cidr" contains "10.0.0.0/8"

#################
# Get Drop Entry by CIDR
#################

# Get existing IPv4 drop entry - Success
GET http://localhost/v1/drop/192.168.1.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.cidr" == "192.168.1.0/24"
jsonpath "$.tag" == "test-drop-ipv4"
jsonpath "$.expiration" == {{expiration_ts}}

# Get existing IPv6 drop entry - Success
GET http://localhost/v1/drop/2001:db8::/32
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.cidr" == "2001:db8::/32"
jsonpath "$.tag" == "test-drop-ipv6"
jsonpath "$.expiration" == {{expiration_ts}}

# Get non-existent drop entry - Should fail
GET http://localhost/v1/drop/203.0.113.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404

# Get entry with invalid IP format - Should fail
GET http://localhost/v1/drop/invalid-ip/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 400

# Get entry with invalid prefix - Should fail
GET http://localhost/v1/drop/192.168.1.0/99
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 400

#################
# Delete Drop Entries
#################

# Delete existing IPv6 drop entry - Success
DELETE http://localhost/v1/drop/2001:db8::/32
Authorization: Bearer {{token}}

HTTP 204

# Verify entry was deleted
GET http://localhost/v1/drop/2001:db8::/32
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404

# Try to delete non-existent entry - Should fail
DELETE http://localhost/v1/drop/203.0.113.0/24
Authorization: Bearer {{token}}

HTTP 404

# Try to delete with invalid CIDR - Should fail
DELETE http://localhost/v1/drop/invalid-ip/24
Authorization: Bearer {{token}}

HTTP 400

# Verify remaining entry count
GET http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" count == 2

#################
# Peer Entries Tests
#################

# Add and remove entries via peer endpoint
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "add",
    "entry": {
      "cidr": "203.0.113.0/24",
      "tag": "peer-test",
      "expiration": {{expiration_ts}}
    }
  },
  {
    "action": "add",
    "entry": {
      "cidr": "198.51.100.0/24",
      "tag": "peer-test-2",
      "expiration": {{expiration_ts}}
    }
  }
]

HTTP 201

# Verify peer entries were added
GET http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" count == 4
jsonpath "$[*].cidr" contains "203.0.113.0/24"
jsonpath "$[*].cidr" contains "198.51.100.0/24"

# Remove entries via peer endpoint
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "remove",
    "entry": {
      "cidr": "203.0.113.0/24",
      "expiration": 0
    }
  }
]

HTTP 201

# Test invalid peer action - Should fail
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "invalid",
    "entry": {
      "cidr": "203.0.113.0/24"
    }
  }
]

HTTP 422

#################
# Authentication Tests
#################

# Test without authorization header - Should fail
GET http://localhost/v1/drop

HTTP 401

# Test with invalid token - Should fail
POST http://localhost/v1/drop
Authorization: Bearer invalid-token
Content-Type: application/json
{
  "cidr": "192.168.2.0/24",
  "tag": "unauthorized-test",
  "expiration": {{expiration_ts}}
}

HTTP 401

#################
# Drop Access Control Tests
#################

# Test that clientrw group CAN access drop endpoints
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-drop-clientrw-user",
  "group": "clientrw"
}

HTTP 201
[Captures]
clientrw_token: jsonpath "$.token"

# clientrw group should have access to list drop entries
GET http://localhost/v1/drop
Authorization: Bearer {{clientrw_token}}

HTTP 200
[Asserts]
jsonpath "$" isCollection

# clientrw group should have access to create drop entries
POST http://localhost/v1/drop
Authorization: Bearer {{clientrw_token}}
Content-Type: application/json
{
  "cidr": "172.16.0.0/16",
  "tag": "clientrw-test",
  "expiration": {{expiration_ts}}
}

HTTP 201

# clientrw group should have access to get drop entries
GET http://localhost/v1/drop/172.16.0.0/16
Authorization: Bearer {{clientrw_token}}

HTTP 200

# clientrw group should have access to delete drop entries
DELETE http://localhost/v1/drop/172.16.0.0/16
Authorization: Bearer {{clientrw_token}}

HTTP 204

# Test that clientro group has limited access to drop endpoints
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-drop-clientro-user",
  "group": "clientro"
}

HTTP 201
[Captures]
clientro_token: jsonpath "$.token"

# clientro group should have read access to list drop entries
GET http://localhost/v1/drop
Authorization: Bearer {{clientro_token}}

HTTP 200
[Asserts]
jsonpath "$" isCollection

# clientro group should have read access to get drop entries
GET http://localhost/v1/drop/192.168.1.0/24
Authorization: Bearer {{clientro_token}}

HTTP 200

# clientro group should be denied write access to create drop entries
POST http://localhost/v1/drop
Authorization: Bearer {{clientro_token}}
Content-Type: application/json
{
  "cidr": "172.16.0.0/16",
  "tag": "clientro-denied",
  "expiration": {{expiration_ts}}
}

HTTP 401

# clientro group should be denied write access to delete drop entries
DELETE http://localhost/v1/drop/192.168.1.0/24
Authorization: Bearer {{clientro_token}}

HTTP 401

# Test that monitoring group cannot access drop endpoints
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-drop-monitoring-user",
  "group": "monitoring"
}

HTTP 201
[Captures]
monitoring_token: jsonpath "$.token"

# monitoring group should be denied access to drop endpoints
GET http://localhost/v1/drop
Authorization: Bearer {{monitoring_token}}

HTTP 401

# Test that peering group CAN access drop endpoints
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-drop-peering-user",
  "group": "peering"
}

HTTP 201
[Captures]
peering_token: jsonpath "$.token"

# peering group should have access to peer endpoint
POST http://localhost/v1/drop/peer
Authorization: Bearer {{peering_token}}
Content-Type: application/json
[
  {
    "action": "add",
    "entry": {
      "cidr": "203.0.113.128/25",
      "tag": "peering-test",
      "expiration": {{expiration_ts}}
    }
  }
]

HTTP 201

# peering group should not be able to list entries
GET http://localhost/v1/drop
Authorization: Bearer {{peering_token}}

HTTP 401

# peering group should NOT be able to create individual entries
POST http://localhost/v1/drop
Authorization: Bearer {{peering_token}}
Content-Type: application/json
{
  "cidr": "172.16.1.0/24",
  "tag": "peering-denied",
  "expiration": {{expiration_ts}}
}

HTTP 401

#################
# Cleanup
#################

# Clean up remaining test entries
DELETE http://localhost/v1/drop/192.168.1.0/24
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/drop/10.0.0.0/8
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/drop/198.51.100.0/24
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/drop/203.0.113.128/25
Authorization: Bearer {{token}}
HTTP 204

# Cleanup test users
DELETE http://localhost/v1/client/test-drop-clientrw-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-drop-clientro-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-drop-monitoring-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-drop-peering-user
Authorization: Bearer {{token}}
HTTP 204
