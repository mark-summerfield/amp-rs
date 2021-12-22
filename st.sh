#!/bin/bash
tokei -f -slines -c80 -tPython,Rust -etarget
git st
