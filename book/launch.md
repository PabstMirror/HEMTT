# hemtt launch

Available in: **v0.10.0 Alpha 8**

<code>
hemtt launch
</code>

`hemtt launch` is used to build and launch a dev version of your mod. It will run the [`hemtt dev`](dev.md) command internally after a few checks, and options are passed to the `dev` command.

## Configuration

**hemtt.toml**

```toml
[hemtt.launch]
mods = [
    "450814997", # CBA_A3's Workshop ID
]
parameters = [
    "-skipIntro",           # These parameters are passed to the Arma 3 executable
    "-noSplash",            # They do not need to be added to your list
    "-showScriptErrors",    # You can add additional parameters here
    "-debug",
    "-filePatching",
]
```

### mods

A list of workshop IDs to launch with your mod. These are not subscribed to, and will need to be manually subscribed to in Steam.

### parameters

A list of [Startup Parameters](https://community.bistudio.com/wiki/Arma_3:_Startup_Parameters) to pass to the Arma 3 executable.