# XIVTools
## Ventures: A retainer assistant
Ventures is a simple command line bot that allows you to re-assign your retainers when they're finished with their tasks. It supports any number of retainers, and any length venture for each. Check the -h information for usage information.

For best results, open the retainer window and then minimize the game while in windowed mode.

## Talan: A crafting assistant
[![Talan (beta 2 candidate) crafting 5x Metal Gauntlets for the Crystalarium](http://i3.ytimg.com/vi/neSoWRJTPfE/maxresdefault.jpg)](https://www.youtube.com/watch?v=neSoWRJTPfE)

### Overview
Talan is a crafting assistant designed for max level crafters. Rather than trying to be a crafting
solver like [FFXIV Crafting Optimizer](https://ffxiv-beta.lokyst.net/#/simulator), It reads in FFXIV macros directly,
but executes them using its own engine that has none of the limitations of the in-game macro system.

Special thanks to Clorifex of [GarlandTools](https://garlandtools.org) and Miu of [FFXIV Teamcraft](https://ffxivteamcraft.com)
for various bits of help along the way.

### Features
Talan is in its first beta, but already has a fairly solid set of features
- It has a working GUI that allows a user to queue up tasks made up of any number of crafts
  with different configurations. This includes items that can be crafted by multiple jobs,
  and collectable turn-ins.
- When not running in 'slow mode' it can overall craft 1/4th of a second *faster* per action than
  the in-game macro engine.
- It needs no action keybinds, it operates entirely through the text interface.
- It can parse any variation of FFXIV macros (quoted, unquoted, with wait, without wait)
- It can change gearsets to allow chaining of commands and crafts.
- It will use both NQ and HQ materials, prioritizing NQ.
- It uses XIVapi.com to lookup and configure crafts.

### Caveats / Known Issues
- Right now Talan assumes the basic keybinds for Confirm, Cancel, Up, Down, Left, Right, Backward,
  and Forward are default. If necessary, these can be changed in `xiv/src/ui.rs`, but I recommend using the game defaults unless you rebound numpad.
- The XIV UI is fininky, so it's best to run the game in windowed mode and minimize it before starting a run
  of tasks to ensure you can't mistakenly alter the modality of the game's UI. Even moving the mouse over
  the window can interrupt the game's idea of which input method is being used.
- Talan will look in $the current working directory for a directory named `macros` to search for macros.
  If the `macros` directory doesn't exist then you'll experience a crash.
- **If you are crafting collectables you must make sure your last action finishes the craft. Additional actions will presently cause the window input to fail. Sim your rotations!**

### Roadmap
Talan is still under active development with the following roadmap in mind:
- Configuring Role Actions if they are found in a macro. (possibly unnecessary in Shadowbringers)
- Automatically adding prereq crafts to the list.
- Caching item searches, or having a local item database, so that we can run if XIVApi is down.
- Using and refreshing crafter food.
- Using and refreshing crafter tea.
- Adding a progress bar / window for the GUI rather than dropping to console.

### Usage
Talan is largely controlled via the GUI. Run with -v or -vv for various amounts of debug info.

To use it you will first need to install Rust via the [installation instructions](https://www.rust-lang.org/en-US/install.html). If given the option, you need the 2018 channel. Rust likely requires Visual Studio Community edition to be installed.
