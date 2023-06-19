#!/bin/bash

DECODE="unique_triangles"
IMAGES=""

for file in ./images/*
do
  IMAGES+="-i ${file} "
done

#echo ${IMAGES}

cargo run -- ${IMAGES} -d ${DECODE} > output.txt
