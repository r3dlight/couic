set -e

# Start couic-static
nohup /mnt/release/couic-static -c /tmp/couic.toml &
# Wait for couic startup
sleep 2

# Extract token for reuse
TOKEN=$(cat /tmp/rbac/clients/couicctl.toml | grep '^token' | sed 's/token = "\(.*\)"/\1/')

# General Integration tests (original tests)
cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test \
    /mnt/tests/integration/clients.hurl \
    /mnt/tests/integration/drop.hurl \
    /mnt/tests/integration/ignore.hurl \
    /mnt/tests/integration/stats.hurl \
    /mnt/tests/integration/sets.hurl

# New edge case and validation tests
cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test \
    /mnt/tests/integration/edge_cases.hurl \
    /mnt/tests/integration/metadata.hurl \
    /mnt/tests/integration/malformed_requests.hurl \
    /mnt/tests/integration/cross_policy.hurl \
    /mnt/tests/integration/http_methods.hurl

# Temp dirs for sets
SETS_DIR="/tmp/sets/ignore"

# Create set1.couic with duplicates
cat > "$SETS_DIR/set1.couic" <<EOF
192.168.1.0/24
10.0.0.0/8
2001:db8::/32
192.168.1.0/24
1.1.1.0/24
EOF
chmod 600 "$SETS_DIR/set1.couic"

# Create set2.couic with overlap
cat > "$SETS_DIR/set2.couic" <<EOF
10.0.0.0/8
172.16.0.0/12
192.168.1.0/24
2001:db8::/32
EOF
chmod 600 "$SETS_DIR/set2.couic"

# Create a normal entry file (not a set)
cat > "$SETS_DIR/normal_entry.txt" <<EOF
8.8.8.8/32
EOF

# Run initial hurl tests (should see deduplication and correct tags)
cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test /mnt/tests/integration/sets/step1.hurl

# Rename set1.couic to set1-renamed.couic
mv "$SETS_DIR/set1.couic" "$SETS_DIR/set1-renamed.couic"
sleep 1

# Run hurl to check tags updated after rename
cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test /mnt/tests/integration/sets/step2.hurl

# Now add 8.8.8.8/32 to set2.couic (override normal entry)
echo "8.8.8.8/32" >> "$SETS_DIR/set2.couic"

# Run hurl to check override and tag
cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test /mnt/tests/integration/sets/step3.hurl

# Remove set2.couic and reload
rm "$SETS_DIR/set2.couic"

# Run hurl to check entries removed or reverted
cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test /mnt/tests/integration/sets/step4.hurl

#################
# Additional Set Edge Case Tests
#################

# Step 5: Test empty set file
cat > "$SETS_DIR/empty.couic" <<EOF
EOF
chmod 600 "$SETS_DIR/empty.couic"

cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test /mnt/tests/integration/sets/step5.hurl

rm "$SETS_DIR/empty.couic"

# Step 6: Test set file with comments and empty lines
cat > "$SETS_DIR/valid_cidrs.couic" <<EOF
# This is a comment
8.8.8.0/24

# Another comment
1.0.0.0/24

# Empty lines above should be ignored
EOF
chmod 600 "$SETS_DIR/valid_cidrs.couic"

cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test /mnt/tests/integration/sets/step6.hurl

rm "$SETS_DIR/valid_cidrs.couic"

# Step 7: Test set file with mixed valid and invalid CIDRs
cat > "$SETS_DIR/mixed.couic" <<EOF
9.9.9.0/24
invalid-cidr
also-not-valid
EOF
chmod 600 "$SETS_DIR/mixed.couic"

cd /tmp && hurl --variable expiration_ts=2547914495 \
    --variable token=$TOKEN \
    --jobs 1 --unix-socket /tmp/couic.sock --test /mnt/tests/integration/sets/step7.hurl

rm "$SETS_DIR/mixed.couic"

# Cleanup remaining set files
rm -f "$SETS_DIR/set1-renamed.couic"

echo "All integration tests completed successfully!"
