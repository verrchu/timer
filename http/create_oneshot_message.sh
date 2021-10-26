#!/bin/sh

http POST $ENDPOINT/api/v1/createOneshotMessage << EOF
{
    "message_id": "$(uuidgen)",
    "data": "${DATA:=test}",
    "schedule": {
        "at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    }
}
EOF