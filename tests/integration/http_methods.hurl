#################
# HTTP Method Edge Cases Tests
#################

# Tests for unsupported HTTP methods and method handling

#################
# Unsupported Methods on Policy Endpoints
#################

# PUT on /v1/drop (not supported)
PUT http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": "put-test",
  "expiration": 2547914495
}

HTTP 405

# PATCH on /v1/drop (not supported)
PATCH http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": "patch-test"
}

HTTP 405

# PUT on /v1/drop/{ip}/{prefix} (not supported)
PUT http://localhost/v1/drop/192.168.1.0/24
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "tag": "updated-tag"
}

HTTP 405

# PATCH on /v1/drop/{ip}/{prefix} (not supported)
PATCH http://localhost/v1/drop/192.168.1.0/24
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "tag": "patched-tag"
}

HTTP 405

#################
# Unsupported Methods on Client Endpoints
#################

# PUT on /v1/client (not supported)
PUT http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-client",
  "group": "clientro"
}

HTTP 405

# PATCH on /v1/client (not supported)
PATCH http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-client"
}

HTTP 405

# PUT on /v1/client/{name} (not supported)
PUT http://localhost/v1/client/couicctl
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "group": "admin"
}

HTTP 405

# PATCH on /v1/client/{name} (not supported)
PATCH http://localhost/v1/client/couicctl
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "group": "admin"
}

HTTP 405

#################
# Unsupported Methods on Stats Endpoints
#################

# POST on /v1/stats (not supported - stats is read-only)
POST http://localhost/v1/stats
Authorization: Bearer {{token}}
Content-Type: application/json
{}

HTTP 405

# PUT on /v1/stats (not supported)
PUT http://localhost/v1/stats
Authorization: Bearer {{token}}
Content-Type: application/json
{}

HTTP 405

# DELETE on /v1/stats (not supported)
DELETE http://localhost/v1/stats
Authorization: Bearer {{token}}

HTTP 405

# POST on /v1/metrics (not supported)
POST http://localhost/v1/metrics
Authorization: Bearer {{token}}
Content-Type: application/json
{}

HTTP 405

# DELETE on /v1/metrics (not supported)
DELETE http://localhost/v1/metrics
Authorization: Bearer {{token}}

HTTP 405

#################
# Unsupported Methods on Sets Endpoints
#################

# GET on /v1/sets/reload (not supported - reload is POST only)
GET http://localhost/v1/sets/reload
Authorization: Bearer {{token}}

HTTP 405

# PUT on /v1/sets/reload (not supported)
PUT http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json
{}

HTTP 405

# DELETE on /v1/sets/reload (not supported)
DELETE http://localhost/v1/sets/reload
Authorization: Bearer {{token}}

HTTP 405

#################
# Unsupported Methods on Peer Endpoints
#################

# GET on /v1/drop/peer (not supported - peer is POST only)
GET http://localhost/v1/drop/peer
Authorization: Bearer {{token}}

HTTP 405

# PUT on /v1/drop/peer (not supported)
PUT http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[]

HTTP 405

# DELETE on /v1/drop/peer (not supported)
DELETE http://localhost/v1/drop/peer
Authorization: Bearer {{token}}

HTTP 405

#################
# OPTIONS Method (CORS preflight)
#################

# OPTIONS on /v1/drop
OPTIONS http://localhost/v1/drop
Authorization: Bearer {{token}}

HTTP 405

# OPTIONS on /v1/stats
OPTIONS http://localhost/v1/stats
Authorization: Bearer {{token}}

HTTP 405

#################
# HEAD Method
#################

# HEAD on /v1/drop (should work like GET but no body)
HEAD http://localhost/v1/drop
Authorization: Bearer {{token}}

HTTP 200

# HEAD on /v1/stats
HEAD http://localhost/v1/stats
Authorization: Bearer {{token}}

HTTP 200

# HEAD on /v1/metrics
HEAD http://localhost/v1/metrics
Authorization: Bearer {{token}}

HTTP 200

#################
# POST on GET-only Endpoints
#################

# POST on /v1/stats/tags/drop (not supported)
POST http://localhost/v1/stats/tags/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{}

HTTP 405

# POST on /v1/stats/tags/ignore (not supported)
POST http://localhost/v1/stats/tags/ignore
Authorization: Bearer {{token}}
Content-Type: application/json
{}

HTTP 405
