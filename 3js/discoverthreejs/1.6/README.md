# 1.6

## Summary
This is a threejs application that introduces responsive scenes.

## Terms
* `jaggies` - The "lines" are not straight. This is because everything on screen is composed of square pixels.
* `anti-aliasing` - A technique that smoothens lines out. 

## Information
* Avoid scenes with lots of long, thin lines. They will either look jagged, or need lots of effort to straighten out with anti-aliasing.
* There are big differences between desktop GPUs and mobile devices. For the web especially, mobile matters more. 
* Event listeners are necessary to add responsiveness to user input. We listen for an event, and it will trigger a callback function.

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