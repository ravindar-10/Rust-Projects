Tmux Commands
Session Management
    • tmux new -s session_name → Create a new session
    • tmux ls → List all sessions
    • tmux attach -t session_name → Attach to a session
    • tmux detach (Ctrl+b d) → Detach from the current session
    • tmux kill-session -t session_name → Kill a specific session
    • tmux kill-server → Kill all sessions
Window Management
    • Ctrl+b c → Create a new window
    • Ctrl+b , → Rename the current window
    • Ctrl+b w → List all windows
    • Ctrl+b n → Switch to the next window
    • Ctrl+b p → Switch to the previous window
    • Ctrl+b [0-9] → Switch to a specific window by number
    • Ctrl+b & → Close the current window
Pane Management
    • Ctrl+b % → Split pane vertically
    • Ctrl+b " → Split pane horizontally
    • Ctrl+b o → Switch between panes
    • Ctrl+b x → Close the current pane
    • Ctrl+b z → Toggle zoom for the current pane
    • Ctrl+b { → Move pane left
    • Ctrl+b } → Move pane right
Resizing Panes
    • Ctrl+b : resize-pane -D → Resize pane down
    • Ctrl+b : resize-pane -U → Resize pane up
    • Ctrl+b : resize-pane -L → Resize pane left
    • Ctrl+b : resize-pane -R → Resize pane right
Copy Mode (Scrolling & Copying Text)
    • Ctrl+b [ → Enter copy mode
    • Up/Down/PgUp/PgDn → Scroll within copy mode
    • Space → Start selection
    • Enter → Copy selection
    • Ctrl+b ] → Paste copied text
Miscellaneous
    • Ctrl+b d → Detach from the session
    • Ctrl+b ? → Show keybindings help
    • tmux source-file ~/.tmux.conf → Reload the tmux config file
Let me know if you need any specific tmux commands! 🚀

