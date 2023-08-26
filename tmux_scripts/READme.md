tmux new-session "tmux source-file ~/tmux_scripts/web"

put this folder in ~/Documents

# alias:

add in ~/.bashrc

source ~/.bashrc

```bash
#tmux web
alias tweb='tmux new-session "tmux source-file ~/Documents/tmux_scripts/web"'

#tmux rust
alias trust='tmux new-session "tmux source-file ~/Documents/tmux_scripts/rust"'

#tmux random some 5 tabs
alias trand='tmux new-session "tmux source-file ~/Documents/tmux_scripts/rand"'


alias v="nvim"
```
