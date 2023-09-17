
```
rm -rf target Cargo.lock
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/lib/x86_64-linux-gnu/pkgconfig/
echo $PKG_CONFIG_PATH
cargo run --features bevy/dynamic_linking
```

[bevy book](https://bevyengine.org/learn/book/getting-started/ecs/)

[bevy cheatbook](https://bevy-cheatbook.github.io/features/coords.html)

[bevy examples](https://github.com/bevyengine/bevy/tree/latest/examples#examples)

[rust book](https://doc.rust-lang.org/book/)

## building for web

```
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/treasure-dragon-roar.wasm
```

## Audio

Audio is complicated because it needs to be initiated by a user action.  in my html I made a button call the init method, otherwise sound would not play.  

## Canvas

Also, the js code created by `wasm-bindgen` creates a canvas element on init. I used javascript to position the canvas and other elements after the init to make the page layout correct. 

## Attaching to window

this is a gotcha: in script with type=module i need to explicitly attach functions to the window to make them available to buttons

https://stackoverflow.com/questions/49338193/how-to-use-code-from-script-with-type-module

## assets

the wasm binary assumes the assets are in the folder beside it.  I wasn't able to figure out how to customize where it looks for the assets after its built, so to satisfy it, in my static site generated i did a copy passthrough from where the assets really are to where it's trying to find them.

## html example

here is the html for an example webpage that renders a video game.  (This is actually a template so it's missing some sections like head):

```html
---
layout: base-empty
permalink: /games/treasure-dragon-roar.html
---

<script type="module">
  import init from "/assets/wasm/treasure-dragon-roar/treasure-dragon-roar.js";

  window.start = () => {
    init().catch((error) => {
      if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
          throw error;
      }
    }).then(() => {
      const canvas = document.querySelector("canvas");
      const div = document.getElementsByClassName("mx-auto")[0];
      const startButton = document.getElementById("start");
      startButton.remove();
      div.appendChild(canvas);
      canvas.classList.add("mx-auto")
      const instructions = document.getElementById("instructions");
      div.appendChild(instructions);
      instructions.classList.remove("hidden");
      canvas.focus();
    })
  }
</script>

<button id="start" class="border-2 rounded-lg p-8 text-2xl font-semibold bg-sky-300 hover:bg-green-300 mx-auto block mt-[30vh]" onclick="start()">Start</button>

<p id="instructions" class="hidden max-w-prose mx-auto text-white">
  Explore the landscape to find the treasure. Press the arrow keys to move; Press [Enter] grab the treasure and to drop the treasure; Press [Space] to ROAR!
</p>
```