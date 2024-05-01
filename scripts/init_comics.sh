#!/usr/bin/env bash
set -eo pipefail

# Global variable for quiet mode
QUIET=""

# Function to log messages with optional indentation
log() {
    # Terminate early if quiet mode is enabled
    [[ -n "$QUIET" ]] && return

    # Add spaces for indentation based on the current function call depth
    local indentation=""
    for ((i=0; i<FUNCNEST; i++)); do
        indentation+="  "
    done

    # Print message to stderr
    >&2 echo "${indentation}>> $1"
}

# Function to remove old comics directory
function remove_comics {
    log "Removing old comics directory"
    rm -rf comics
}

# Function to create new directories
function create_dirs {
    log "Creating new directories"
    mkdir -p comics/in comics/out
}

# Function to create new files
function create_files {
    log "Creating new files"
    touch "comics/in/Alpha 001 (2024).cbz"
    touch "comics/in/Beta 000 (2024).cbr"
}

# Parse command-line options
while getopts "q" opt; do
    case ${opt} in
        q ) # Quiet mode, suppress log output
            QUIET="true"
            ;;
        \? )
            echo "Invalid option: $OPTARG" 1>&2
            exit 1
            ;;
    esac
done
shift $((OPTIND -1))

# Main script logic
remove_comics
create_dirs
create_files
