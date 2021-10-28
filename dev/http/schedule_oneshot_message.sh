#!/bin/sh

http POST $ENDPOINT/api/v1/scheduleOneshotMessage << EOF
{
    "message_id": "${MESSAGE_ID:-$(uuidgen)}",
    "user_id": "${USER_ID:-$(uuidgen)}",
    "content": "${CONTENT:-test}",
    "schedule": {
        "at": "$(date -v ${DELTA:-+1d} -u +"%Y-%m-%dT%H:%M:%SZ")"
    }
}
EOF
