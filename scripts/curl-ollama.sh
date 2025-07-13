#!/bin/sh

curl \
    -X POST \
    http://127.0.0.1:11434/api/generate \
    -H "Content-Type: application/json" \
    -d '{"model":"llama3","prompt":"Just reply: Hello, world!","system":"You are a coding assistant","temperature":0.10000000149011612}'
