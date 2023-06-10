#!/bin/bash

DECODE="unique_triangles"
IMAGES=""

for file in ./images/*
do
  IMAGES+=${file}
done

echo ${IMAGES}

cargo run -- -i ${IMAGES} -d ${DECODE}
