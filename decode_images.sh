#!/bin/bash

DECODE="unique_triangles"

for file in ./images/*
do
  cargo run -- -i ${file} -d ${DECODE} -p
done
