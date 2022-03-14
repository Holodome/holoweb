#!/usr/bin/env bash

curl -d '{"name":"Example","contents":"Contents"}' -H "Content-Type: application/json" -X POST 127.0.0.1:8080/api/post