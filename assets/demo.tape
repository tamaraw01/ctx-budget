# VHS documentation: https://github.com/charmbracelet/vhs

Output assets/demo.gif

Require ctx-budget

Set Margin 20
Set MarginFill "#674EFF"
Set BorderRadius 10

Set FontSize 16
Set FontFamily "JetBrains Mono"
Set Width 900
Set Height 600
Set Theme "Catppuccin Mocha"
Set WindowBar Colorful

# Setup the environment quietly
Hide
Type "cd assets/demo-env" Enter
Type "clear" Enter
Show

# Typing the command
Sleep 1s
Type "ctx-budget . --model claude-sonnet-4 --limit 5"
Sleep 500ms
Enter

# Wait for scan to finish (it's instant)
Sleep 3s

# Show exclusion filtering
Type "clear" Enter
Sleep 500ms
Type "ctx-budget . --exclude-dirs 'docs' --output json | jq ."
Sleep 500ms
Enter

Sleep 4s
