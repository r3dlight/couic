#################
# Edge Cases and Validation Tests
#################

#################
# CIDR Validation Edge Cases
#################

# Test CIDR with host bits set (non-canonical form) - should normalize or reject
# 192.168.1.100/24 has host bits set, canonical form is 192.168.1.0/24
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.100/24",
  "tag": "host-bits-test",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "192.168.1.0/24"

# Cleanup
DELETE http://localhost/v1/drop/192.168.1.0/24
Authorization: Bearer {{token}}
HTTP 204

# Test IPv6 CIDR with host bits set
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "2001:db8::1/32",
  "tag": "ipv6-host-bits-test",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "2001:db8::/32"

# Test minimum valid IPv4 prefix (/1)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "128.0.0.0/1",
  "tag": "min-prefix-ipv4",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "128.0.0.0/1"

# Cleanup
DELETE http://localhost/v1/drop/128.0.0.0/1
Authorization: Bearer {{token}}
HTTP 204

# Test maximum valid IPv4 prefix (/32)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "1.2.3.4/32",
  "tag": "max-prefix-ipv4",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "1.2.3.4/32"

# Cleanup
DELETE http://localhost/v1/drop/1.2.3.4/32
Authorization: Bearer {{token}}
HTTP 204

# Test minimum valid IPv6 prefix (/1)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "8000::/1",
  "tag": "min-prefix-ipv6",
  "expiration": {{expiration_ts}}
}

HTTP 201

[Asserts]
jsonpath "$.cidr" == "8000::/1"

# Cleanup
DELETE http://localhost/v1/drop/8000::/1
Authorization: Bearer {{token}}
HTTP 204

# Test maximum valid IPv6 prefix (/128)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "2001:db8::1/128",
  "tag": "max-prefix-ipv6",
  "expiration": {{expiration_ts}}
}

HTTP 201
[Asserts]
jsonpath "$.cidr" == "2001:db8::1/128"

# Cleanup
DELETE http://localhost/v1/drop/2001:db8::1/128
Authorization: Bearer {{token}}
HTTP 204

# Test invalid prefix /33 for IPv4 (out of range)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/33",
  "tag": "invalid-prefix-33",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test invalid prefix /129 for IPv6 (out of range)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "2001:db8::/129",
  "tag": "invalid-prefix-129",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test CIDR without prefix (missing /prefix)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0",
  "tag": "no-prefix",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test CIDR with negative prefix
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/-1",
  "tag": "negative-prefix",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test CIDR with non-numeric prefix
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/abc",
  "tag": "non-numeric-prefix",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test empty CIDR
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "",
  "tag": "empty-cidr",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test CIDR with extra slashes
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0//24",
  "tag": "double-slash",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test CIDR with trailing slash
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24/",
  "tag": "trailing-slash",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test CIDR with whitespace
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": " 192.168.1.0/24 ",
  "tag": "whitespace-cidr",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test IPv4-mapped IPv6 address
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "::ffff:192.168.1.0/120",
  "tag": "ipv4-mapped-ipv6",
  "expiration": {{expiration_ts}}
}

HTTP 201

#################
# Tag Validation Edge Cases
#################

# Test tag with maximum allowed length (64 chars)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.1.0.0/16",
  "tag": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "expiration": {{expiration_ts}}
}

HTTP 201

# Cleanup
DELETE http://localhost/v1/drop/10.1.0.0/16
Authorization: Bearer {{token}}
HTTP 204

# Test tag exceeding maximum length (65 chars)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.2.0.0/16",
  "tag": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test tag with invalid characters (special chars not in pattern)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.3.0.0/16",
  "tag": "invalid@tag#test",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test tag with spaces
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.4.0.0/16",
  "tag": "tag with spaces",
  "expiration": {{expiration_ts}}
}

HTTP 422

# Test tag with valid special characters (should work: _ -)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.5.0.0/16",
  "tag": "ValiDtag_name-test",
  "expiration": {{expiration_ts}}
}

HTTP 201

# Cleanup
DELETE http://localhost/v1/drop/10.5.0.0/16
Authorization: Bearer {{token}}
HTTP 204

# Test empty tag (should be allowed as null)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.6.0.0/16",
  "tag": "",
  "expiration": {{expiration_ts}}
}

HTTP 201

# Cleanup
DELETE http://localhost/v1/drop/10.6.0.0/16
Authorization: Bearer {{token}}
HTTP 204

# Test tag with unicode characters
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.7.0.0/16",
  "tag": "tag-with-émoji",
  "expiration": {{expiration_ts}}
}

HTTP 422

#################
# Expiration Edge Cases
#################

# Test expiration = 0 (never expire)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.10.0.0/16",
  "tag": "never-expire",
  "expiration": 0
}

HTTP 201
[Asserts]
jsonpath "$.expiration" == 0

# Cleanup
DELETE http://localhost/v1/drop/10.10.0.0/16
Authorization: Bearer {{token}}
HTTP 204

# Test very large expiration timestamp (year 2100)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.11.0.0/16",
  "tag": "far-future",
  "expiration": 4102444800
}

HTTP 201
[Asserts]
jsonpath "$.expiration" == 4102444800

# Cleanup
DELETE http://localhost/v1/drop/10.11.0.0/16
Authorization: Bearer {{token}}
HTTP 204

# Test expiration as string (should fail - wrong type)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.12.0.0/16",
  "tag": "string-expiration",
  "expiration": "2547914495"
}

HTTP 400

# Test expiration as float (should fail or truncate)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.13.0.0/16",
  "tag": "float-expiration",
  "expiration": 2547914495.5
}

HTTP 400

#################
# Path Parameter Edge Cases
#################

# Test get with IPv4 prefix mismatch (asking for /25 when entry is /24)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "172.20.0.0/24",
  "tag": "prefix-mismatch-test",
  "expiration": {{expiration_ts}}
}

HTTP 201

# This should 404 - different prefix
GET http://localhost/v1/drop/172.20.0.0/25
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404

# This should 200 - correct prefix
GET http://localhost/v1/drop/172.20.0.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200

# Cleanup
DELETE http://localhost/v1/drop/172.20.0.0/24
Authorization: Bearer {{token}}
HTTP 204

# Test invalid policy name
GET http://localhost/v1/invalid_policy
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 400

# Test policy name with uppercase (should fail - enum is lowercase)
GET http://localhost/v1/DROP
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 400

#################
# Per-tag Stats Endpoint Tests
#################

# Test per-tag drop stats endpoint
GET http://localhost/v1/stats/tags/drop
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tags" isCollection

# Test per-tag ignore stats endpoint
GET http://localhost/v1/stats/tags/ignore
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tags" isCollection

#################
# Per-tag Stats Access Control
#################

# Create monitoring user for access control tests
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-edge-monitoring-user",
  "group": "monitoring"
}

HTTP 201
[Captures]
monitoring_token: jsonpath "$.token"

# Monitoring group should have access to per-tag stats
GET http://localhost/v1/stats/tags/drop
Authorization: Bearer {{monitoring_token}}

HTTP 200

GET http://localhost/v1/stats/tags/ignore
Authorization: Bearer {{monitoring_token}}

HTTP 200

# Create clientro user
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-edge-clientro-user",
  "group": "clientro"
}

HTTP 201
[Captures]
clientro_token: jsonpath "$.token"

# clientro group should be denied access to per-tag stats
GET http://localhost/v1/stats/tags/drop
Authorization: Bearer {{clientro_token}}

HTTP 401

GET http://localhost/v1/stats/tags/ignore
Authorization: Bearer {{clientro_token}}

HTTP 401

# Cleanup test users
DELETE http://localhost/v1/client/test-edge-monitoring-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-edge-clientro-user
Authorization: Bearer {{token}}
HTTP 204
