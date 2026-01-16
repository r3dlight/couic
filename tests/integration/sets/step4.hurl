# Step 4: Remove set2.couic and check entries removed or reverted

# Reload sets after removing set2.couic
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 201
[Asserts]
jsonpath "$.reload_status" == "OK"

# 8.8.8.8/32 should no longer have set2.couic tag (should be absent or have normal tag)
GET http://localhost/v1/ignore/8.8.8.8/32
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404

# 172.16.0.0/12 should be gone (was only in set2)
GET http://localhost/v1/ignore/172.16.0.0/12
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 404
