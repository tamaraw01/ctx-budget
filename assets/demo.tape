# Demo for ctx-budget - tape for charmbracelet/vhs
# https://github.com/charmbracelet/vhs

Output "assets/demo.gif"

Require ctx-budget

Set FontSize 15
Set FontFamily "JetBrains Mono"
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set Padding 10
Set Margin 16
Set BorderRadius 8
Set WindowBar Colorful

Hide
Type "cd assets/demo-env" Enter
Type "clear" Enter
Show

Sleep 800ms
Type "ctx-budget . --model gpt-6-astra --limit 5" Enter
Sleep 3500ms

Type "clear" Enter
Sleep 400ms
Type "ctx-budget . --output json | jq . | head -n 25" Enter
Sleep 3500ms
