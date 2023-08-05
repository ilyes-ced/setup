all my config files

## rices
<details>
  <summary>Rose-Pine</summary>
  
  
![not_found](/images/pine-rose-rice.png)
![not_found](/images/pine-rose-rice2.png)
![not_found](/images/pine-rose-rice3.png)
![not_found](/images/pine-rose-rice4.png)
![not_found](/images/pine-rose-rice5.png)

gnome extentions:
    blur my shell
    dash to dock
    gnome clipboard
    logo menu
    pop shell
    remove app menu
    top bar organizer
    unite
    user theme
    aylur's widgets
    workspace matrix
    Rounded Window Corners


themes for: 

    gnome (/gnome/rices/rose-pine)
    	gtk3
    	gtk4
    	gnome-shell
    	icons
    
	vscode (extentions + /vscodium/vscode_config)
	nvim (mainly from nvChad)
	text editor (gnome/rices/rose-pine/usr....../style.xml)
	alacritty (themes need to be set manually in the .config/alacritty folder)
	ranger (themes need to be set manually in the .config/alacritty folder)
	fierfox (extention)
	tmux
	duckduckgo


## tmux
i install the catppuccin theme for tmux because it looks better then replace the color scheme in theme plugins folder with
```conf
thm_bg="#191724"
thm_fg="#e0def4"
thm_cyan="#9ccfd8"
thm_black="#191724"
thm_gray="#26233a"
thm_magenta="#c4a7e7"
thm_pink="#eb6f92"
thm_red="#de3967"
thm_green="#9ccfd8"
thm_yellow="#f6c177"
thm_blue="#31748f"
thm_orange="#ebbcba"
thm_black4="#6e6a86"
```

plugins folder path example: 

the full config is ~/.config/tmux/plugins/<theme_plugin_name>/color_scheme.tmuxtheme

in my case it was ~/.config/tmux/plugins/tmux/catppuccin-mocha.tmuxtheme

## Font
jetbrains nerd font: https://www.nerdfonts.com/font-downloads

## duckduckgo

1. Visit <https://duckduckgo.com>
2. Right click and select the “Inspect” button.
3. Select the “Console” tab.
4. If you are using Firefox, type `allow pasting`.
5. Enter one of the following scripts

for rose-pine:
```js
const theme = [
	'1=-1', 'at=-1', 'ao=-1', 'aq=-1', 'ak=-1', 'ax=-1', 'av=1', 'ap=-1', 'au=-1', 'ay=b', 'ae=-1', '18=1',
	'7=191724', 'j=191724', '9=9ccfd8', 'x=31748f', 'aa=c4a7e7', '8=e0def4', '21=191724',
];

for (const item of theme) {
	document.cookie = `${item}; max-age=126144000; samesite=lax; secure`;
}
```

## zsh
syntax highlighting
suggestions


## aditional
random pokemon on terminal launch: https://github.com/Findarato/pokemon-colorscripts
pipes.sh: yay -S pipes.sh

</details>
