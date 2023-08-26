all my config files

## gnome rices
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

color scripts: yay -S shell-color-scripts 
</details>




















<details>
  <summary>Catppuccin</summary>
  
  
![not_found](/images/catppuccin1.png)
![not_found](/images/catppuccin2.png)
![not_found](/images/catppuccin3.png)

## duckduckgo

```js
(function() {
  const colors = {
    latte: {
      base: "#eff1f5",
      blue: "#1e66f5",
      lavender: "#7287fd",
      mantle: "#e6e9ef",
      rosewater: "#dc8a78",
      text: "#4c4f69",
    },
    frappe: {
      base: "#303446",
      blue: "#8caaee",
      lavender: "#babbf1",
      mantle: "#292c3c",
      rosewater: "#f2d5cf",
      text: "#c6d0f5",
    },
    macchiato: {
      base: "#24273a",
      blue: "#8aadf4",
      lavender: "#b7bdf8",
      mantle: "#1e2030",
      rosewater: "#f4dbd6",
      text: "#cad3f5",
    },
    mocha: {
      base: "#1e1e2e",
      blue: "#89b4fa",
      lavender: "#b4befe",
      mantle: "#181825",
      rosewater: "#f5e0dc",
      text: "#cdd6f4",
    }
  };
  const flavour = window.prompt("Choose a theme:", "mocha");
  const blueLinks = confirm("Use blue links?");

  const ct = colors[flavour];
  const theme = [
    `21=${ct.mantle}`,
    `7=${ct.base}`,
    `8=${ct.text}`,
    `9=${blueLinks ? ct.blue : ct.rosewater}`,
    `aa=${ct.lavender}`,
    `ae=${flavour == "latte" ? -1 : ct.base}`,
    `j=${ct.mantle}`,
    `x=${blueLinks ? ct.blue : ct.rosewater}`,
  ];

  for (const item of theme) {
    document.cookie = `${item}; max-age=126144000; samesite=lax; secure`;
  }
})();

```

</details>






## i3 rices


for the pywal themes



<details>
  <summary>cyberpunk</summary>
  
  
![not_found](/images/i3_cyberpunk1.png)
![not_found](/images/i3_cyberpunk2.png)
![not_found](/images/i3_cyberpunk3.png)


## duckduckgo

```js
const theme = [
	'1=-1', 'at=-1', 'ao=-1', 'aq=-1', 'ak=-1', 'ax=-1', 'av=1', 'ap=-1', 'au=-1', 'ay=b', 'ae=-1', '18=1',
	'7=0c0618', 'j=0c0618', '9=A115A4', 'x=4ABEE4', 'aa=A39AF0', '8=e3cde6', '21=0c0618',
];

for (const item of theme) {
	document.cookie = `${item}; max-age=126144000; samesite=lax; secure`;
}
```


</details>