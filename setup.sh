#!/bin/bash

createdb khalzam
psql -f initdb.sql khalzam
