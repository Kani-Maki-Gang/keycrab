#! /usr/bin/bash
rm -r ../ff/assets || true
rm ../ff/index.html || true
rm ../ff/index.js || true
dioxus build --release && rm ../dist/index.html && cp -r ../dist/* ../ff && cp ../base/* ../ff
