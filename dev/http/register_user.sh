#!/bin/sh

http POST $ENDPOINT/api/v1/registerUser << EOF
{
    "user_id": "${USER_ID:-$(uuidgen)}"
}
EOF
