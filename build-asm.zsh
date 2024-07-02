#!/bin/zsh

as -arch arm64 -o "dependency-chain.o" "dependency-chain.asm"
as -arch arm64 -o "load-store-ports.o" "load-store-ports.asm"
