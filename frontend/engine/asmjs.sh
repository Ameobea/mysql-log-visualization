cargo rustc --target=asmjs-unknown-emscripten --release --verbose -- -Z print-link-args -C\
  link-args="-v -g -O0 --js-library ./src/library_minutiaeclient.js -s TOTAL_MEMORY=268435456 -s ASSERTIONS=1 -s NO_EXIT_RUNTIME=1 -s ALLOW_MEMORY_GROWTH=1"
