#!/bin/bash

cargo run <<EOT;
START: inc loop
loop: dec loop STOP
list
EOT
