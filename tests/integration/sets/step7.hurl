# Step 7: Set file with mixed valid and invalid CIDRs
# This test runs after creating a set file with some invalid entries

# Reload sets - should return error
POST http://localhost/v1/sets/reload
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 422
