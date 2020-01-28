#!/usr/bin/bash

rm -R docs/*
cp -R html docs
mv docs/html docs/game
cp -R target/doc/chirperjax/* docs
