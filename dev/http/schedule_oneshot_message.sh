#!/bin/sh

http POST $ENDPOINT/api/v1/scheduleOneshotMessage << EOF
{
    "message_id": "$(uuidgen)",
    "user_id": "$(uuidgen)",
    "content": "$CONTENT",
    "schedule": {
        "at": "$(date -v $DELTA -u +"%Y-%m-%dT%H:%M:%SZ")"
    }
}
EOF
