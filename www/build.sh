#!/bin/sh

set -ex

yarn run build:grammar
hugo --minify
