# muu hero demo: direct execution + interactive selector
Source docs/tapes/config.tape
Output docs/tapes/hero.gif
Require muu

# Setup: copy demo.toml as muu.toml in the repo
Hide
Type "cp docs/demo.toml muu.toml && clear"
Enter
Sleep 1s
Show

# 1. Direct execution (no args)
Type "muu hello"
Sleep 500ms
Enter
Sleep 1.5s

# 2. Direct execution (with arg)
Type "muu greet world"
Sleep 500ms
Enter
Sleep 1.5s

# 3. Interactive selector
Type "muu"
Sleep 500ms
Enter
Sleep 1s

# Fuzzy filter
Type "re"
Sleep 1s

# Select recent
Down
Sleep 300ms
Down
Sleep 500ms
Enter
Sleep 4s

# Cleanup
Hide
Type "rm muu.toml"
Enter
Show
