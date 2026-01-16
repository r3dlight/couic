#################
# Sets API Tests
#################

# Test sets reload - Success
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.reload_status" == "OK"

# Test sets reload multiple times (should work)
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

#################
# Authentication Tests
#################

# Test without authorization header - Should fail
POST http://localhost/v1/sets/reload

HTTP 401

# Test with invalid token - Should fail
POST http://localhost/v1/sets/reload
Authorization: Bearer invalid-token

HTTP 401

#################
# Sets Access Control Tests
#################

# Test that clientrw group can reload sets
# First create a clientrw user token
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-sets-clientrw-user",
  "group": "clientrw"
}

HTTP 201
[Captures]
clientrw_token: jsonpath "$.token"

# clientrw group should access to sets reload
POST http://localhost/v1/sets/reload
Authorization: Bearer {{clientrw_token}}

HTTP 201

# Test that clientro group cannot reload sets
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-sets-clientro-user",
  "group": "clientro"
}

HTTP 201
[Captures]
clientro_token: jsonpath "$.token"

# clientro group should be denied access to sets reload
POST http://localhost/v1/sets/reload
Authorization: Bearer {{clientro_token}}

HTTP 401

# Test that monitoring group cannot reload sets
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-sets-monitoring-user",
  "group": "monitoring"
}

HTTP 201
[Captures]
monitoring_token: jsonpath "$.token"

# monitoring group should be denied access to sets reload
POST http://localhost/v1/sets/reload
Authorization: Bearer {{monitoring_token}}

HTTP 401

# Test that peering group cannot reload sets
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-sets-peering-user",
  "group": "peering"
}

HTTP 201
[Captures]
peering_token: jsonpath "$.token"

# peering group should be denied access to sets reload
POST http://localhost/v1/sets/reload
Authorization: Bearer {{peering_token}}

HTTP 401

# Test that admin group CAN reload sets (using original token which should be admin)
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

#################
# Content-Type Tests
#################

# Test sets reload without Content-Type header (should still work)
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

# Test sets reload with different Content-Type
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: text/plain

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

#################
# Cleanup
#################

# Cleanup test users
DELETE http://localhost/v1/client/test-sets-clientrw-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-sets-clientro-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-sets-monitoring-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-sets-peering-user
Authorization: Bearer {{token}}
HTTP 204
