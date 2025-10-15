#!/bin/bash

curl -X POST -H 'Content-Type: application/json' -d '{"message": "Nice to meet you", "price": 100000.99}' localhost:3000/mirror_body_json
