# Uncomment this to disable title bars
for_window [class=".*"] border pixel 1
smart_borders on

# Gaps (Outer gaps are added to the inner gaps)
gaps inner 0
gaps outer 0

# audio keys 
bindsym XF86AudioNext 			exec --no-startup-id playerctl play #"mpc next"
bindsym XF86AudioPrev 			exec --no-startup-id playerctl play #"mpc prev"
bindsym XF86AudioPlay 			exec --no-startup-id playerctl play-pause #"mpc toggle"
bindsym XF86AudioStop 			exec --no-startup-id playerctl stop #"mpc stop"