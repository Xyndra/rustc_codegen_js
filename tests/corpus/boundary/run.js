// Node driver: wire `__js_imports` (the import table the JS bundle reads)
// before importing the generated module, then call `run()` and print.
globalThis.__js_imports = {
  js_now: () => 42.0,
  js_log: (s) => {}, // mirror of the native oracle's no-op js_log
};
const m = await import(process.argv[2]);
console.log(m.run());
