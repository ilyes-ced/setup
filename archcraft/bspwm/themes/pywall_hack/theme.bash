# ------------------------------------------------------------------------------
# Copyright (C) 2020-2023 Aditya Shakya <adi1090x@gmail.com>
#
# Hack Theme
# ------------------------------------------------------------------------------


. "${HOME}/.cache/wal/colors.sh"

# Colors
background= "$background"
foreground= "$foreground"
color0= "$color0"
color1= "$color1"
color2= "$color2"
color3= "$color3"
color4= "$color4"
color5= "$color5"
color6= "$color6"
color7= "$color7"
color8= "$color8"
color9= "$color9"
color10= "$color10"
color11= "$color11"
color12= "$color12"
color13= "$color13"
color14= "$color14"
color15= "$color15"

accent='#007B82'
color_red='#ff0000'
color_green='#00ff00'
color_yellow='#ffff00'

light_value='0.05'
dark_value='0.30'

# Wallpaper
wdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
wallpaper="$wdir/wallpaper"

# Polybar
polybar_font='Iosevka Nerd Font:size=10;3'

# Rofi
rofi_font='Iosevka 10'
rofi_icon='Papirus-Apps'

# Terminal
terminal_font_name='JetBrainsMono Nerd Font'
terminal_font_size='10'

# Geany
geany_colors='hack.conf'
geany_font='JetBrains Mono 10'

# Appearance
gtk_font='Noto Sans 9'
gtk_theme='Hack'
icon_theme='Hack'
cursor_theme='LyraB'

# Dunst
dunst_width='300'
dunst_height='80'
dunst_offset='10x36'
dunst_origin='top-right'
dunst_font='Iosevka 10'
dunst_border='1'
dunst_separator='1'

# Picom
picom_backend='glx'
picom_corner='0'
picom_shadow_r='14'
picom_shadow_o='0.30'
picom_shadow_x='-12'
picom_shadow_y='-12'
picom_blur_method='none'
picom_blur_strength='0'

# Bspwm
bspwm_fbc="$accent"
bspwm_nbc="$background"
bspwm_abc="$color5"
bspwm_pfc="$color2"
bspwm_border='1'
bspwm_gap='10'
bspwm_sratio='0.50'
