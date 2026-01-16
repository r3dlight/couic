# Step 1: Initial sets creation, deduplication, and tag checks

# Reload sets to apply initial set files
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

# List ignore entries and check deduplication and tags
GET http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" isCollection
jsonpath "$[*].cidr" contains "192.168.1.0/24"
jsonpath "$[*].cidr" contains "10.0.0.0/8"
jsonpath "$[*].cidr" contains "2001:db8::/32"
jsonpath "$[*].cidr" contains "172.16.0.0/12"
jsonpath "$[*].cidr" contains "1.1.1.0/24"
# Should not have duplicate 192.168.1.0/24
jsonpath "$[*].cidr" count == 5

# Check tags for set entries (should end with .couic)
GET http://localhost/v1/ignore/192.168.1.0/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith ".couic"

GET http://localhost/v1/ignore/10.0.0.0/8
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith ".couic"

GET http://localhost/v1/ignore/2001:db8::/32
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith ".couic"

GET http://localhost/v1/ignore/172.16.0.0/12
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith ".couic"
