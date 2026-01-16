# Step 2: Rename set and check tag update

# Reload sets after renaming set1.couic to set1-renamed.couic
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

# List ignore entries and check tags reflect new set name
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
jsonpath "$[*].cidr" count == 5

# Check tag for renamed set (should end with set1-renamed.couic)
GET http://localhost/v1/ignore/1.1.1.1/24
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith "set1-renamed.couic"

# set2.couic entries should still have set2.couic tag
GET http://localhost/v1/ignore/172.16.0.0/12
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith "set2.couic"
