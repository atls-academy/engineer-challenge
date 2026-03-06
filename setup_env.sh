#!/bin/bash

# This script is used to set up the environment variables from the .env file.
# Usage: source setup_env.sh

if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
    echo "Environment variables loaded from .env"
else
    echo ".env file not found. Please create it first."
fi
