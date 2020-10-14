# Like coreutils sleep(1), but with progress

Have u ever done

```bash
$ sleep 1h && systemctl suspend
```

and then wondered how long it's left? throw sleep in the trash bin, a new competitor has entered the match:

![example](nap.gif)

```bash
$ nap 1h && systemctl suspend
⠴ [00:00:03] [>-----------------------------------------------------------] [00:00:10]
# sum of arguments is supported, just like with GNU sleep(1):
$ nap 1d 4h 22m 11s .5h .01m
⠒ [00:00:01] [>-----------------------------------------------------------] [28:52:11]
# and this
$ nap infinity
⠴
```

everything is emitted on stderr. uses colors unless piped. [set CLICOLOR_FORCE to 1][1] to force color output.

that's all folks, see you next episode of useless crap u didn't know u wanted

[1]: https://bixense.com/clicolors/
