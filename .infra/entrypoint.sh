#!/usr/bin/bash

function hard_entrypoint_init() {
    npm i
    npm build
    npm run
}

function seek_entrypoint_rep() {
    cd $(ls | grep -r 'entrypoint.sh' ./)
    ./entrypoint.sh
}

function out_auth_ctl() {
    echo seek_entrypoint_rep()
    echo hard_entrypoint_init()
}