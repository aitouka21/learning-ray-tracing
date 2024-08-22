#!/usr/bin/env bash

cargo run --release > image.ppm && kitty icat image.ppm
