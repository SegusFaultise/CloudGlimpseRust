# CloudGlimpse "Rust"
Web/desktop based .las file visualizer

# Building wasm
```console
wasm-pack build --release --target web
```
# Running
```console
python3 serve.py
```
# Features
- [x] pan/orbit camera
- [ ] resizable points
- [ ] point colours based on classification
- [ ] segmentation based on classification (e.g. move/remove buildings, foliage etc...)
- [ ] stream/chunk las file loading
- [ ] laz file format support
