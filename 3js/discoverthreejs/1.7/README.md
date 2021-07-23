# 1.7

## Summary
This is a threejs application that introduces the illusion of movement into scenes.

## Terms
 

## Information
* renderer.render outputs a still image. In order to display "movement", the renderer needs to continuously output still images to the screen, and extremely quickly.
* 60 frames per second means 60 still images are produced and shown per second. This means calculations in between the frames being shown must occur within 16 milliseconds.
* The `animation loop` is similar to the game loop.
* Because every object needs to be updated within the inbetween time between frames, the updates to objects should be as minimal as possible for good performance.
* Frame rates of films/tv shows are fixed. 24fps for films, 30fps for tv.
* If the frame rate is not explicitly set, threejs will by default use the hardware defined refresh rate of the screen.
    * A 60Hz refresh rate should mean 60fps, a 90Hz refresh rate should mean 90 fps, etc.
* 60fps does not mean update loops are completed in `exactly` 16.66 ms. The delta is just very, very close to 16.66 ms, and that small difference is not noticable to humans. 

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