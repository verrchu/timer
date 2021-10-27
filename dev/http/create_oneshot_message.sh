#!/bin/sh

http POST $ENDPOINT/api/v1/createOneshotMessage << EOF
{
    "message_id": "$(uuidgen)",
    "data": "$DATA",
    "schedule": {
        "at": "$(date -v $DELTA -u +"%Y-%m-%dT%H:%M:%SZ")"
    }
}
EOF
