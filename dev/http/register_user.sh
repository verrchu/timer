#!/bin/sh

http POST $ENDPOINT/api/v1/registerUser << EOF
{
    "user_id": "$(uuidgen)"
}
EOF
