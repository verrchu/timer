#!/bin/sh

http POST $ENDPOINT/api/v1/scheduleOneshotMessage << EOF
{
    "user_id": "$USER_ID",
    "content": "${CONTENT:-test}",
    "schedule": {
        "in": ${IN:-86400}
    }
}
EOF
