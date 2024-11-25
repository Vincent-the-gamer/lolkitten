import chalk from 'chalk'
import info from '../package.json'
import { fromFile, fromPipe, init, options, println } from './index'
// @ts-expect-error missing types
import minimist from 'minimist'
// @ts-expect-error missing types
import supportsColor from 'supports-color'

const args = minimist(process.argv.slice(2), {
  alias: {
    v: 'version',
    h: 'help',
    f: 'force',
    p: 'spread',
    F: 'freq',
    S: 'seed',
    a: 'animate',
    d: 'duration',
    D: 'debug',
    s: 'speed',
  },
})

function rand(max: number) {
  return Math.floor(Math.random() * (max + 1))
}

function help() {
  const help = `
Usage: lolkitten [OPTION]... [FILE]...

Concatenate FILE(s), or standard input, to standard output.
With no FILE, or when FILE is -, read standard input.

    --spread, -p <f>:   Rainbow spread (default: 8.0)
      --freq, -F <f>:   Rainbow frequency (default: 0.3)
      --seed, -S <i>:   Rainbow seed, 0 = random (default: 0)
       --animate, -a:   Enable psychedelics
  --duration, -d <i>:   Animation duration (default: 12)
     --speed, -s <f>:   Animation speed (default: 20.0)
         --force, -f:   Force color even when stdout is not a tty
       --version, -v:   Print version and exit
         --debug, -D:   Display error messages, if any (default: false)
          --help, -h:   Show this message

Examples:
  lolkitten f - g     Output f's contents, then stdin, then, g's contents.
  lolkitten           Copy standard input to standard output.

Report lolkitten bugs to <https://github.com/Vincent-the-gamer/lolkitten/issues>
lolkitten home page: <https://github.com/Vincent-the-gamer/lolkitten/>`

  let i: number = 20
  const o: number = rand(256)
  const lines = help.split('\n')

  for (const line in lines) {
    i -= 1
    options.seed = o + i
    println(lines[line])
  }

  process.exit()
}

function version() {
  if (options.seed === 0) {
    options.seed = rand(256)
  }

  println(`lolkitten ${info.version} (c) 2024-PRESENT Vincent-the-gamer`)

  process.exit()
}

function initLolkitten(args: any) {
  if (args.force) {
    chalk.level = supportsColor.supportsColor({ isTTY: true }).level
  }

  if (args.help) {
    help()
  }

  if (args.version) {
    version()
  }

  if (args.spread) {
    options.spread = args.spread
  }

  if (args.freq) {
    options.freq = args.freq
  }

  if (args.seed) {
    options.seed = args.seed
  }

  if (args.animate) {
    options.animate = true
  }

  if (args.duration) {
    options.duration = args.duration
  }

  if (args.speed) {
    options.speed = args.speed
  }

  if (args.debug) {
    options.debug = Boolean(args.debug)
  }

  init()

  if (args._.length === 0) {
    if (options.seed === 0) {
      options.seed = rand(256)
    }

    fromPipe()
  }
  else {
    let promise: Promise<any> = Promise.resolve()
    args._.forEach((file: string) => {
      if (file === '-') {
        promise = promise.then(() => fromPipe())
      }
      else {
        promise = promise.then(() => fromFile(file))
      }
    })
  }
}

initLolkitten(args)
