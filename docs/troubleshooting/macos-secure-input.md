# Shortcuts stopped working on macOS: Secure Input

Quill shows "Can't record shortcuts right now: macOS Secure Input is blocking key events" when macOS has switched on **Secure Input**. This is a macOS security feature. While it is on, macOS hides your keystrokes from every app except the one you are typing in, so Quill cannot see your shortcut. It is not a Quill bug, and any app that listens for global shortcuts is affected.

Secure Input normally turns on only while you are typing in a password field. Sometimes an app or a system window leaves it on.

## Find what is holding it

In Terminal, run:

```
ioreg -l -w 0 | grep -o 'kCGSSessionSecureInputPID"=[0-9]*'
```

If it prints a number, that is the process that holds Secure Input. Look it up with `ps -p <number> -o comm=`. If nothing prints, Secure Input is off.

## Common causes and fixes

- **`loginwindow`:** a system password or authorization window is open or stuck, often after granting a permission in System Settings. Close any open password prompt (try Command+Tab to find it), then lock and unlock your Mac (Control+Command+Q). If it stays on, log out and back in.
- **Terminal:** in the Terminal menu bar, turn off **Terminal → Secure Keyboard Entry**.
- **iTerm2:** turn off **iTerm2 → Secure Keyboard Entry**.
- **A password field:** click out of any focused password field (in a browser, a password manager, or an app).
- **Password managers and screen-lock tools:** quit and reopen them if they keep Secure Input on.

Quill checks again by itself. Once Secure Input is off, the warning disappears and the Change button works.
