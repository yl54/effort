# 1.13

## Summary
This is a threejs application that introduces loading 3D models.

## Terms
* `glTF` - Graphics language transmission format. It is a format used to load 3D images.

## Information
* glTF files are relatively large, so its wise to asynchronously download those files. 
	* If done synchronously, it may hold up the entire thread.
* glTF files come in two formats
    * Standard `.gltf` files are uncompressed and may come with an extra `.bin` data file.
	* Binary `.glb` files include all data in one single file.

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