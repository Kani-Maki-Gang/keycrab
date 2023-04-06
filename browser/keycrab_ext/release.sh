#! /usr/bin/bash

rm -r ../ff/assets || true
rm ../ff/index.html || true
rm ../ff/index.js || true
rm ../ff/tailwind.css || true

dioxus build --release \
&& rm dist/index.html \
&& cp index.html ../ff \
&& cp -r dist/assets ../ff \
&& cp dist/index.js ../ff \
&& cp dist/tailwind.css ../ff
