#!/bin/bash

# Generate fortune and clean up for JSON
fortune=$(fortune-kind | tr '\n\r	' ' ' | tr -d '"')

# Read workflow template
workflow=$(cat workflow-template.json)

# Insert fortune into workflow
echo "${workflow//<fortune>/$fortune}" > workflow.json
