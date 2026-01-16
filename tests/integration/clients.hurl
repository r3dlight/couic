#################
# Client API Tests
#################

# List all clients (couicctl)
GET http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" isCollection

#################
# Create Client
#################

# Create a new client - Success
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-client-1",
  "group": "clientrw"
}

HTTP 201
[Asserts]
jsonpath "$.name" == "test-client-1"
jsonpath "$.group" == "clientrw"

# Create another client
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-client-2",
  "group": "clientro",
  "scopes": ["read"]
}

HTTP 201

# Try to create duplicate client - Should fail
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-client-1",
  "group": "admin",
  "scopes": ["read"]
}

HTTP 422

# Try to create client with malformed name - Should fail
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-client%*1",
  "group": "admin",
  "scopes": ["read"]
}

HTTP 422

# Try to create client with malformed group - Should fail
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-client-x",
  "group": "g",
  "scopes": ["read"]
}

HTTP 422

#################
# List Clients (after creation)
#################

GET http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$" count == 3
jsonpath "$[*].name" contains "couicctl"
jsonpath "$[*].name" contains "test-client-1"
jsonpath "$[*].name" contains "test-client-2"

#################
# Get Client by Name
#################

# Get existing client - Success
GET http://localhost/v1/client/test-client-1
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.name" == "test-client-1"
jsonpath "$.group" == "clientrw"

# Get non-existent client - Should fail
GET http://localhost/v1/client/non-existent-client
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404

#################
# Delete Client
#################

# Delete existing client - Success
DELETE http://localhost/v1/client/test-client-2
Authorization: Bearer {{token}}

HTTP 204

# Verify client was deleted
GET http://localhost/v1/client/test-client-2
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404

# Try to delete non-existent client - Should fail
DELETE http://localhost/v1/client/non-existent-client
Authorization: Bearer {{token}}

HTTP 404

# Verify remaining client count
GET http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" count == 2

#################
# Authentication Tests
#################

# Test without authorization header - Should fail
GET http://localhost/v1/client

HTTP 401

# Test with invalid token - Should fail
GET http://localhost/v1/client
Authorization: Bearer invalid-token

HTTP 401

#################
# Cleanup
#################

# Delete remaining test client
DELETE http://localhost/v1/client/test-client-1
Authorization: Bearer {{token}}

HTTP 204

#################
# Client Access Control Tests
#################

# Test that clientrw group cannot access client endpoints
# First create a clientrw user token
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-clientrw-user",
  "group": "clientrw"
}

HTTP 201
[Captures]
clientrw_token: jsonpath "$.token"

# clientrw group should be denied access to list clients
GET http://localhost/v1/client
Authorization: Bearer {{clientrw_token}}

HTTP 401

# clientrw group should be denied access to get client
GET http://localhost/v1/client/couicctl
Authorization: Bearer {{clientrw_token}}

HTTP 401

# clientrw group should be denied access to create client
POST http://localhost/v1/client
Authorization: Bearer {{clientrw_token}}
Content-Type: application/json
{
  "name": "test-denied-client",
  "group": "admin"
}

HTTP 401

# Test that clientro group cannot access client endpoints
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-clientro-user",
  "group": "clientro"
}

HTTP 201
[Captures]
clientro_token: jsonpath "$.token"

# clientro group should be denied access to list clients
GET http://localhost/v1/client
Authorization: Bearer {{clientro_token}}

HTTP 401

# clientro group should be denied access to delete client
DELETE http://localhost/v1/client/couicctl
Authorization: Bearer {{clientro_token}}

HTTP 401

# Test that monitoring group cannot access client endpoints
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-monitoring-user",
  "group": "monitoring"
}

HTTP 201
[Captures]
monitoring_token: jsonpath "$.token"

# monitoring group should be denied access to list clients
GET http://localhost/v1/client
Authorization: Bearer {{monitoring_token}}

HTTP 401

# monitoring group should be denied access to create client
POST http://localhost/v1/client
Authorization: Bearer {{monitoring_token}}
Content-Type: application/json
{
  "name": "test-denied-client",
  "group": "admin"
}

HTTP 401

# Test that peering group cannot access client endpoints
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-peering-user",
  "group": "peering"
}

HTTP 201
[Captures]
peering_token: jsonpath "$.token"

# peering group should be denied access to list clients
GET http://localhost/v1/client
Authorization: Bearer {{peering_token}}

HTTP 401

# peering group should be denied access to get client
GET http://localhost/v1/client/couicctl
Authorization: Bearer {{peering_token}}

HTTP 401

# peering group should be denied access to delete client
DELETE http://localhost/v1/client/couicctl
Authorization: Bearer {{peering_token}}

HTTP 401

# Test that admin group CAN access client endpoints (using original token which should be admin)
GET http://localhost/v1/client
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
jsonpath "$" isCollection

# Admin can create clients
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-admin-client",
  "group": "clientro"
}

HTTP 201

# Admin can get clients
GET http://localhost/v1/client/test-admin-client
Authorization: Bearer {{token}}

HTTP 200

# Admin can delete clients
DELETE http://localhost/v1/client/test-admin-client
Authorization: Bearer {{token}}

HTTP 204

# Cleanup test users
DELETE http://localhost/v1/client/test-clientrw-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-clientro-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-monitoring-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-peering-user
Authorization: Bearer {{token}}
HTTP 204