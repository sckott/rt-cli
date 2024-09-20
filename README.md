# rt-cli

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

rt = R test

```
rt help
```

```
Usage: rt <command> [<args>]

CLI tool for running tests in R packages

Options:
  --help            display usage information

Commands:
  dir               Test an R package using testthat
  file              Test a single file using testthat
  list              List test files in an R package
  r-vers            List available versions of R
```

## Install

`cargo install --git https://github.com/sckott/rt-cli.git`

OR

Run `make` after cloning this repo
