(define RLISP_HOME (env-var "RLISP_HOME"))

; Imports the file prefixed with RLISP_HOME
(define-macro (import-lib name)
    `(import ,(format "#{RLISP_HOME}/#{name}")))

(import-lib "stdio.rl")
(import-lib "stdlib.rl")
(import-lib "repl.rl")
(import-lib "error.rl")
(import-lib "map.rl")

; Define interactive entry point
(define interactive-start start-repl)