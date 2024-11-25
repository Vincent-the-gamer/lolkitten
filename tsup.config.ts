import type { Options } from 'tsup'

export default <Options> {
  entryPoints: [
    "./src/{cli,index}.ts",
  ],
  clean: true,
  platform: "node",
  format: ['esm'],
  dts: true,
  onSuccess() {
    console.info("🙏 Build succeeded!")
  }
}