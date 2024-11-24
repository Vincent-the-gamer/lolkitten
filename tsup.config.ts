import type { Options } from 'tsup'

export default <Options> {
  entryPoints: [
    "./src/cli.ts",
    "./src/index.ts"
  ],
  clean: true,
  format: ['esm',"cjs"],
  dts: true,
  minify: true,
  // compatible with __dirname in cjs and import.meta.url in mjs.
  shims: true
}