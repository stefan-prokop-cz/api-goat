#!/bin/bash

echo 'Apply DB migrations';
diesel migration run;
./api-goat;
