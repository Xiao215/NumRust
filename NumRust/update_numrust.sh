#!/bin/sh
maturin build --release
yes Y | find ./target/wheels -name "*.whl" -exec pip3 uninstall {} \;
find ./target/wheels -name "*.whl" -exec pip3 install {} \;
