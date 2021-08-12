# 1.14

## Summary
This is a threejs application that introduces 3js animations.

## Terms
* `Keyframe` - a drawing or shot that defines the starting and ending points of any smooth transition. These are called frames because their position in time is measured in frames on a strip of film or on a digital video editing timeline.
* `Animation Clip` - a collection of a number of keyframes attached to a single object.

## Information
* Simple animations can be done within a few frames, complex animations may need many frames to complete.
* Keyframes consist of 3 pieces of info: time, property, and value.
    * "at {some time} seconds, a {a property} is {a value}".
* An animation needs minimum 2 keyframes.
* As animation clips get more detailed, they tend to get tied to particular models.
 
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