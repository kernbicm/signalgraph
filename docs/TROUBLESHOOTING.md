# SignalGraph troubleshooting

Three things break 95% of the time, and they break on a first run. This
page fixes all three plus the common pipeline issues.

## Camera preview is black / nothing happens

1. Make sure you approved camera permission when the OS dialog popped
   up. On Windows 11, open **Settings → Privacy & security → Camera**
   and confirm SignalGraph (or Tauri, in dev mode) is allowed.
2. Pick a camera explicitly in the dropdown. The default choice is the
   first device the OS reports, which is not always the one you want.
3. Close other apps that might own the camera (Zoom, Teams, OBS,
   browser tabs). Only one process can hold a camera at a time on most
   platforms.
4. Click the retry button in the permission overlay. Persistent failure
   usually means a driver issue; reboot or reinstall the camera driver.

## No OSC packets are arriving at my target

Run these checks in order - each one eliminates an entire class of
problem.

1. **Run the OSC loopback test.** Lab tab → OSC loopback panel → run.
   If this fails, the Rust OSC sender or your loopback interface is
   broken and nothing downstream will work.
2. **Check the packet monitor.** Monitor tab → packet stream. If
   packets show up here but your receiver sees nothing, the problem is
   on the network side of the wire.
3. **Verify host and port.** Most OSC clients default to 127.0.0.1 on
   port 9000. Cross-machine setups need the target's IP, not 127.0.0.1.
4. **Verify the OSC address.** Many apps match on exact address
   strings, so `/demo/hand/x` and `/demo/hand/X` are different routes.
5. **Confirm the sink is enabled.** In the inspector, the
   `enabled` checkbox on `osc_out` must be ticked. Active-sinks panel
   in the Monitor tab will show disabled sinks grayed out.
6. **Firewall.** On Windows, the first time a dev build tries to send
   outbound UDP the firewall prompts. Allow it on the private network
   at minimum.

## Patch fails to load or runtime shows red errors

- "unknown node kind …" - you loaded a patch authored for a newer
  version. Update SignalGraph.
- "node X missing input on port 'input'" - the patch references an
  edge that was deleted. Delete the orphan node or reconnect it.
- "graph contains a cycle" - v1 only supports DAGs. Break the cycle.
- "tracker_signal 'src' missing signal_path" - open the inspector and
  pick a signal.
- "signal mp.xxx not available" - your patch references a signal the
  worker is not emitting. Either switch to fake source to iterate, or
  pick a different signal path that you can see in the source explorer.

## Worker crashes on start

- In dev mode the sidecar path is `python worker/main.py`. If your
  Python executable is not on PATH the spawn fails with
  `spawn failed ... No such file or directory`. Either put Python on
  PATH or activate your venv first.
- If you see `opencv import failed` or `mediapipe import failed` on
  stderr but the app still works, the worker is falling back to fake
  mode on purpose - install `opencv-python` and `mediapipe` inside the
  worker venv to use the real tracker.
- In a packaged build the worker runs from the bundled sidecar binary.
  If that binary was not produced (see `docs/PACKAGING.md`), the app
  will tell you "worker binary or worker/main.py not found".

## Source explorer only shows `worker.fake` signals

You are in fake-source mode. Click **start tracker worker** in the Lab
tab. If the worker then transitions to `error`, check the
troubleshooting section above about the worker crash.

## The app runs but values look stuck

- In the status strip at the top, check `last frame ms`. If it is
  frozen, the worker stopped producing frames. Restart it.
- If it is updating but values are identical, your smoothing alpha is
  too low. Try 0.25 or higher.
- Make sure you are moving inside the source's expected range. The
  calibration workflow captures min/max from a live capture window so
  you do not have to guess.
