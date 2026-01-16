#################
# Malformed Request Tests
#################

# Tests for handling invalid JSON and malformed request bodies

#################
# Malformed JSON Bodies
#################

# Test completely invalid JSON
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
```
{not valid json}
```

HTTP 400

# Test JSON with trailing comma
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
```
{
  "cidr": "192.168.1.0/24",
  "tag": "test",
  "expiration": 2547914495,
}
```

HTTP 400

# Test JSON with single quotes instead of double quotes
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
```
{'cidr': '192.168.1.0/24', 'tag': 'test', 'expiration': 2547914495}
```

HTTP 400

# Test empty request body
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 400

# Test null request body
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
```
null
```

HTTP 400

# Test array instead of object
# See issue: https://github.com/serde-rs/json/issues/1059
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
```
["192.168.1.0/24", "test", 2547914495]
```

HTTP 201

# Test string instead of object
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
```
"192.168.1.0/24"
```

HTTP 400

# Test number instead of object
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
```
12345
```

HTTP 400

#################
# Missing Required Fields
#################

# Test missing cidr field
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "tag": "missing-cidr",
  "expiration": 2547914495
}

HTTP 400

# Test missing expiration field
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": "missing-expiration"
}

HTTP 400

# Test empty object (missing all required fields)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{}

HTTP 400

#################
# Wrong Field Types
#################

# Test cidr as number
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": 1921681024,
  "tag": "cidr-as-number",
  "expiration": 2547914495
}

HTTP 400

# Test cidr as array
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": ["192.168.1.0", 24],
  "tag": "cidr-as-array",
  "expiration": 2547914495
}

HTTP 400

# Test cidr as object
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": {"ip": "192.168.1.0", "prefix": 24},
  "tag": "cidr-as-object",
  "expiration": 2547914495
}

HTTP 400

# Test cidr as null
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": null,
  "tag": "cidr-as-null",
  "expiration": 2547914495
}

HTTP 400

# Test cidr as boolean
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": true,
  "tag": "cidr-as-boolean",
  "expiration": 2547914495
}

HTTP 400

# Test tag as number
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": 12345,
  "expiration": 2547914495
}

HTTP 400

# Test tag as array
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": ["tag1", "tag2"],
  "expiration": 2547914495
}

HTTP 400

# Test tag as object
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "192.168.1.0/24",
  "tag": {"name": "mytag"},
  "expiration": 2547914495
}

HTTP 400

#################
# Unknown/Extra Fields
#################

# Test with unknown extra field (should be ignored or rejected)
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "cidr": "10.200.0.0/16",
  "tag": "extra-field-test",
  "expiration": {{expiration_ts}},
  "unknown_field": "should be ignored",
  "another_unknown": 12345
}

HTTP 201

# Cleanup if created
DELETE http://localhost/v1/drop/10.200.0.0/16
Authorization: Bearer {{token}}
HTTP 204

#################
# Malformed Peer Requests
#################

# Test peer with invalid action
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "update",
    "entry": {
      "cidr": "192.168.1.0/24",
      "expiration": 2547914495
    }
  }
]

HTTP 422

# Test peer with missing action
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "entry": {
      "cidr": "192.168.1.0/24",
      "expiration": 2547914495
    }
  }
]

HTTP 422

# Test peer with missing entry
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "add"
  }
]

HTTP 422

# Test peer with empty array
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[]

HTTP 201

# Test peer with object instead of array
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "action": "add",
  "entry": {
    "cidr": "192.168.1.0/24",
    "expiration": 2547914495
  }
}

HTTP 422

# Test peer with mixed valid/invalid entries
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {
    "action": "add",
    "entry": {
      "cidr": "10.201.0.0/16",
      "expiration": {{expiration_ts}}
    }
  },
  {
    "action": "invalid_action",
    "entry": {
      "cidr": "10.202.0.0/16",
      "expiration": {{expiration_ts}}
    }
  }
]

HTTP 422

#################
# Malformed Client Requests
#################

# Test client creation with missing name
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "group": "clientro"
}

HTTP 400

# Test client creation with missing group
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-malformed-client"
}

HTTP 400

# Test client creation with invalid group
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-invalid-group-client",
  "group": "superadmin"
}

HTTP 422

# Test client creation with name as number
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": 12345,
  "group": "clientro"
}

HTTP 400

# Test client creation with group as number
POST http://localhost/v1/client
Authorization: Bearer {{token}}
Content-Type: application/json
{
  "name": "test-group-number",
  "group": 1
}

HTTP 400

#################
# Content-Type Header Tests
#################

# Test POST with wrong Content-Type
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: text/plain
{
  "cidr": "10.204.0.0/16",
  "tag": "wrong-content-type",
  "expiration": {{expiration_ts}}
}

HTTP 400

# Test POST with XML Content-Type
POST http://localhost/v1/drop
Authorization: Bearer {{token}}
Content-Type: application/xml
```
<entry>
  <cidr>10.205.0.0/16</cidr>
  <tag>xml-test</tag>
  <expiration>2547914495</expiration>
</entry>
```

HTTP 400

#################
# Large Payload Tests
#################

# Test peer with many entries (100 entries)
# Note: This tests bulk operation handling
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {"action": "add", "entry": {"cidr": "100.0.0.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.1.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.2.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.3.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.4.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.5.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.6.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.7.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.8.0/24", "expiration": {{expiration_ts}}}},
  {"action": "add", "entry": {"cidr": "100.0.9.0/24", "expiration": {{expiration_ts}}}}
]

HTTP 201

# Cleanup bulk entries
POST http://localhost/v1/drop/peer
Authorization: Bearer {{token}}
Content-Type: application/json
[
  {"action": "remove", "entry": {"cidr": "100.0.0.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.1.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.2.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.3.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.4.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.5.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.6.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.7.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.8.0/24", "expiration": 0}},
  {"action": "remove", "entry": {"cidr": "100.0.9.0/24", "expiration": 0}}
]

HTTP 201
