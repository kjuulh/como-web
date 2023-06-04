#!/bin/bash

tmux new-session -d -s dev

# allow for user input
tmux split-window -h

# leptos
tmux send-keys -t dev "cuddle_cli x leptos:dev" C-m

tmux split-window -v
# tailwind
tmux send-keys -t dev "cuddle_cli x tailwind:watch" C-m

# set user input to first 
tmux select-pane -t 0


tmux attach-session -t dev

