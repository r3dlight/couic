# Step 5: Empty set file handling
# This test runs after creating an empty set file

# Reload sets after creating empty set file
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

# Verify the system still works after loading empty set
GET http://localhost/v1/ignore
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$" isCollection
