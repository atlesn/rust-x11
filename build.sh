#!/bin/sh

cargo build 2>&1 | sed 's/-->//' 1>&2
