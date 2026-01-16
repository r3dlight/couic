#################
# Stats API Tests
#################

# Get stats in JSON format (default)
GET http://localhost/v1/stats
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.drop_cidr_count" isInteger
jsonpath "$.ignore_cidr_count" isInteger
jsonpath "$.xdp" isCollection

#################
# Metrics API Tests
#################

# Get metrics in JSON format (default)
GET http://localhost/v1/metrics
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.drop_cidr_count" isInteger
jsonpath "$.ignore_cidr_count" isInteger
jsonpath "$.xdp" isCollection
jsonpath "$.drop_tags" isCollection
jsonpath "$.ignore_tags" isCollection

# Get metrics in OpenMetrics format
GET http://localhost/v1/metrics?format=prometheus
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
header "Content-Type" contains "application/openmetrics-text"
body contains "# HELP couic_drop_cidr_total Current number of CIDR dropped by couic."
body contains "# TYPE couic_drop_cidr_total gauge"
body contains "couic_drop_cidr_total"
body contains "# HELP couic_ignore_cidr_total Current number of CIDR ignored by couic."
body contains "# TYPE couic_ignore_cidr_total gauge"
body contains "couic_ignore_cidr_total"
body contains "# HELP couic_stats_rx_packets_total Current number of packets handled by XDP."
body contains "# TYPE couic_stats_rx_packets_total counter"
body contains "# HELP couic_stats_rx_bytes_total Current number of bytes handled by XDP."
body contains "# TYPE couic_stats_rx_bytes_total counter"
body contains "# HELP couic_drop_tag_rx_packets_total Number of packets dropped per tag."
body contains "# TYPE couic_drop_tag_rx_packets_total counter"
body contains "# HELP couic_drop_tag_rx_bytes_total Number of bytes dropped per tag."
body contains "# TYPE couic_drop_tag_rx_bytes_total counter"
body contains "# HELP couic_ignore_tag_rx_packets_total Number of packets ignored per tag."
body contains "# TYPE couic_ignore_tag_rx_packets_total counter"
body contains "# HELP couic_ignore_tag_rx_bytes_total Number of bytes ignored per tag."
body contains "# TYPE couic_ignore_tag_rx_bytes_total counter"
body contains "# EOF"

# Test with invalid format parameter (should default to JSON)
GET http://localhost/v1/metrics?format=invalid
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.drop_cidr_count" isInteger
jsonpath "$.ignore_cidr_count" isInteger

# Test with multiple query parameters
GET http://localhost/v1/metrics?format=prometheus&extra=ignored
Authorization: Bearer {{token}}

HTTP 200
[Asserts]
header "Content-Type" contains "application/openmetrics-text"
body contains "couic_drop_cidr_total"

#################
# Authentication Tests
#################

# Test without authorization header - Should fail
GET http://localhost/v1/stats

HTTP 401

# Test with invalid token - Should fail
GET http://localhost/v1/stats
Authorization: Bearer invalid-token

HTTP 401

# Test metrics without auth - Should fail
GET http://localhost/v1/metrics

HTTP 401

# Test metrics OpenMetrics format without auth - Should fail
GET http://localhost/v1/metrics?format=prometheus

HTTP 401

#################
# Edge Cases
#################

# Test with empty format parameter (defaults to JSON)
GET http://localhost/v1/metrics?format=
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.drop_cidr_count" isInteger

# Test case sensitivity for format parameter (case sensitive, defaults to JSON)
GET http://localhost/v1/metrics?format=PROMETHEUS
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.drop_cidr_count" isInteger

# Test with mixed case format parameter (case sensitive, defaults to JSON)
GET http://localhost/v1/metrics?format=Prometheus
Authorization: Bearer {{token}}
Content-Type: application/json

HTTP 200
[Asserts]
header "Content-Type" contains "application/json"
jsonpath "$.drop_cidr_count" isInteger

#################
# Stats Access Control Tests
#################

# Test that clientrw group cannot access stats
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

# clientrw group should be denied access to stats
GET http://localhost/v1/stats
Authorization: Bearer {{clientrw_token}}

HTTP 401

# clientrw group should be denied access to metrics
GET http://localhost/v1/metrics
Authorization: Bearer {{clientrw_token}}

HTTP 401

# Test that clientro group cannot access stats
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

# clientro group should be denied access to stats
GET http://localhost/v1/stats
Authorization: Bearer {{clientro_token}}

HTTP 401

# clientro group should be denied access to metrics
GET http://localhost/v1/metrics
Authorization: Bearer {{clientro_token}}

HTTP 401

# Test that peering group cannot access stats
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

# peering group should be denied access to stats
GET http://localhost/v1/stats
Authorization: Bearer {{peering_token}}

HTTP 401

# peering group should be denied access to metrics
GET http://localhost/v1/metrics
Authorization: Bearer {{peering_token}}

HTTP 401

# Test that monitoring group CAN access stats and metrics
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

# monitoring group should have access to stats
GET http://localhost/v1/stats
Authorization: Bearer {{monitoring_token}}

HTTP 200
[Asserts]
jsonpath "$.drop_cidr_count" isInteger

# monitoring group should have access to metrics
GET http://localhost/v1/metrics
Authorization: Bearer {{monitoring_token}}

HTTP 200
[Asserts]
jsonpath "$.drop_cidr_count" isInteger
jsonpath "$.drop_tags" isCollection
jsonpath "$.ignore_tags" isCollection

# monitoring group should have access to metrics in OpenMetrics format
GET http://localhost/v1/metrics?format=prometheus
Authorization: Bearer {{monitoring_token}}

HTTP 200
[Asserts]
header "Content-Type" contains "application/openmetrics-text"
body contains "couic_drop_cidr_total"

# Cleanup test users
DELETE http://localhost/v1/client/test-clientrw-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-clientro-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-peering-user
Authorization: Bearer {{token}}
HTTP 204

DELETE http://localhost/v1/client/test-monitoring-user
Authorization: Bearer {{token}}
HTTP 204
