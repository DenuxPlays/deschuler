#!/bin/bash

# Initialize a counter
success_count=0

while true; do
    # Run the command and suppress its output
    cargo test > /dev/null 2>&1

    # Check if the command was successful
    if [ $? -eq 0 ]; then
        # If the command was successful, increment the counter
        ((success_count++))
        echo "The script has run successfully $success_count times."
    else
        # If the command failed, terminate the script
        echo "The command failed. The script ran successfully $success_count times before failing."
        exit 1
    fi
done
