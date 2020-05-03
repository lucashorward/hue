This project is divided in the following parts:

## Hue-core
This project handles basic calls to the Hue bridge.
It is able to:
- Get all lights from a bridge.
- Update the state of the lights (on/off, brightness, etc).

## Hue-cli

A Rust CLI that interacts with hue-core to call the Hue Bridge.

## Hue-core-wasm
A shell around Hue-core that allows it to be run as wasm. Interface for js/ts projects.
This is also exported as a npm package: https://www.npmjs.com/package/hue_core_wasm.

## Hue-web-app
An example web app (plain html/js) using the mentioned npm package.