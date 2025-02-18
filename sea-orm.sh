#!/bin/bash

# Load environment variables from .env file if exists
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

# Variables
ENTITY_OUTPUT_DIR="entity/src"

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "DATABASE_URL is not set. Ensure it is defined in the .env file."
    exit 1
fi

# Check if sea-orm-cli is installed
if ! command -v sea-orm-cli &> /dev/null
then
    echo "sea-orm-cli is not installed. Install it with 'cargo install sea-orm-cli'."
    exit 1
fi

# Run the migration refresh
echo "Refreshing migrations..."
sea-orm-cli migrate refresh
if [ $? -ne 0 ]; then
    echo "Migration refresh failed."
    exit 1
fi

# Generate entities
echo "Generating entities..."
sea-orm-cli generate entity -u "$DATABASE_URL" -o "$ENTITY_OUTPUT_DIR"
if [ $? -ne 0 ]; then
    echo "Entity generation failed."
    exit 1
fi

echo "Process completed successfully."

# Run application
echo ""
echo "Before running the application, add Serialize and Deserialize to enum types."