#!/bin/sh

curl \
    -X POST \
    http://192.168.196.10:11434/api/generate \
    -H "Content-Type: application/json" \
    -d '{"model":"gemma3n","prompt":"Just reply: Hello, world!","system":"You are a coding assistant","temperature":0.10000000149011612}'
