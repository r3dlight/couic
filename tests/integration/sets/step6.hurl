# Step 6: Set file with comments and whitespace
# This test runs after creating a set file with comments and empty lines

# Reload sets
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

# Verify only valid CIDRs were loaded (comments and empty lines should be ignored)
GET http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" isCollection
# Should have entries from valid_cidrs.couic
jsonpath "$[*].cidr" contains "8.8.8.0/24"
jsonpath "$[*].cidr" contains "1.0.0.0/24"

# Verify comment lines were not treated as CIDRs
GET http://localhost/v1/ignore/8.8.8.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith "valid_cidrs.couic"
