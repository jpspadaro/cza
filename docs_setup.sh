#!/usr/bin/bash

rm -R docs/*
cp -R html docs
mv docs/html docs/game
cp -R target/doc/* docs
cp README.md docs
