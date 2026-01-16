# Step 3: Override normal entry by adding to set

# Reload sets after adding 8.8.8.8/32 to set2.couic
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

# Check that 8.8.8.8/32 now has set2.couic tag
GET http://localhost/v1/ignore/8.8.8.8/32
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
jsonpath "$.tag" endsWith "set2.couic"
