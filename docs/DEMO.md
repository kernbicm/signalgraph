# SignalGraph acceptance demo

The v1 acceptance demo proves the full loop - camera in, OSC out -
against the packaged build. If this passes, the release is real.

## You will need

- A local OSC receiver on port 9000. [Protokol](https://hexler.net/protokol)
  is the quickest option and works on all three platforms.
- A camera (built-in laptop webcam is fine).
- Decent light. MediaPipe Hands fails quietly in the dark.

## Step by step

1. **Launch the app.** The Lab tab should open automatically.
2. **Allow camera permission** when the OS prompts you.
3. **Pick a camera** from the device dropdown if the default is wrong.
   The app remembers the last selection.
4. **Start the tracker worker.** The status strip should flip the
   worker badge to `running` and the source badge to `worker`. The
   source explorer on the right should start filling with
   `mp.hand.*` signals.
5. **Switch to the Editor tab.**
6. **Load the starter patch:** toolbar → `starter`. You will see four
   nodes: tracker_signal → map_range → smooth → osc_out.
7. **Wave your index finger in front of the camera.** In the editor,
   the `smooth` and `osc_out` nodes should show live values updating.
8. **Switch to the Monitor tab.** The active-sinks panel should list
   `/demo/hand/x` on `127.0.0.1:9000`. The packet stream below should
   be filling up.
9. **Open your OSC receiver** on port 9000. You should see
   `/demo/hand/x` with a float value that tracks your finger position.

## Pass criteria

All of the following must be true:

- [ ] Camera preview renders live video.
- [ ] Worker status is `running` and `last frame ms` is updating.
- [ ] Source explorer shows at least one live
      `mp.hand.*.landmark.*.x` signal.
- [ ] Editor starter patch loads without runtime errors.
- [ ] Packet monitor shows outgoing packets with no send failures.
- [ ] External OSC receiver sees matching values for `/demo/hand/x`.

## Fast sanity demo (post-Phase 2)

If you only want to smoke-test the vertical slice without the graph
editor, the Lab tab's **hardcoded mapping** panel does the same job:

1. Lab tab → hardcoded mapping → set source to
   `mp.hand.left.landmark.index_tip.x`.
2. Flip **enabled** to true.
3. Watch the packet monitor.

This regression path stays available even after the graph editor
exists, so we can always fall back to it if the runtime compilation
ever breaks.
