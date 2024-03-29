# 1.9

## Summary
This is a threejs application that introduces a camera controls plugin, and plugins in general.

## Terms
* `Damping` - This is inertia within the controls. Rather than abruptly stopping, the controls will continue to move a little before stopping.

## Information
* `OrbitControls` is a camera controls plugin. It allows one to move the camera around and zoom.
* q: Why are plugins underneath `examples`?

## Folder structure

* src - holds the source code
* src/world - holds code for the world app
* src/world/components - components are anything that can be placed into the scene, like the cube, the camera, and the scene itself
* src/world/systems - systems are things that operate on components or other systems

## Dependencies
* A web browser that supports WebGL and runs javascript
* `npm`
* `http-server` from `npm`
* `threejs` module file from the threejs Github repo

## How to Run
1. Go to this directory in the cmd line/terminal
2. Run `http-server -c-1`
3. Go to `localhost:8080`

## Notes
This app has to be run on a server because new versions of web browsers have blocked running local files. These files theoretically could be anything, so its assumed to be malicious.