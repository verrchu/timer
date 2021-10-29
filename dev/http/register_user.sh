#!/bin/sh

http POST $ENDPOINT/api/v1/registerUser << EOF
{
    "alias": "$ALIAS"
}
EOF
