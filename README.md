# ghcimd
Loads markdown code blocks into ghci

## Load haskell code block:
```Haskell
import Test.QuickCheck

test n
 | n < 0 = []
 | otherwise = n : test (n-1)
```
Mark a code block as haskell like this:

\`\`\`Haskell


\`\`\`

## Command
`ghcimd path\to\file -p external-packages`
### Ex
`ghcimd .\README.md -p QuickCheck`
This will download QuicCheck using cabal and load every codeblock marked as haskell into ghci



