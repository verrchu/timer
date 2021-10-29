#!/bin/sh

http POST $ENDPOINT/api/v1/scheduleOneshotMessage << EOF
{
    "user_id": "$USER_ID",
    "content": "${CONTENT:-test}",
    "schedule": {
        "at": "$(date -v ${DELTA:-+1d} -u +"%Y-%m-%dT%H:%M:%SZ")"
    }
}
EOF
