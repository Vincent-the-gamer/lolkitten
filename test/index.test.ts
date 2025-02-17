import { options, fromString } from "../src"

options.seed = Math.round(Math.random() * 1000)
options.colors = true

// test of stripping ANSI-escape characters.
const test_ansi_strip = '\x1b[31mThis is red text\x1b[0m'

fromString(test_ansi_strip)