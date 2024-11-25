import { options, fromString } from "../src"

options.seed = Math.round(Math.random() * 1000)
options.colors = true

const arch = `
________________________________________ 
( 绫地宁宁是我老婆。                      )
(               ——沃兹基·硕德           )
 ---------------------------------------- 
        o   ^__^
         o  (oo)________
            (__)\\      )/
                ||----w |
                ||     ||
`

fromString(arch)