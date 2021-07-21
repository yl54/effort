# 1.5

## Summary
This is a threejs application that introduces Transformations, Coordinate Systems, and the Scene Graph.

## Terms
* `Scene graph` - a structure that describes the hierarchy of objects that make up the scenes.
* `Transformation` - a mathematical operation that is used to move objects around 3D space.
	* `Position`
	* `Rotation`
	* `Scaling`
* `World Space` - This is the global scene's coordinate system.
* `Local Space` - This is the individual object's local coordinate system.

## Information
* The `Object3D` class is the base class for lots of things, including meshes, cameras, lights, etc.
	* The class includes the properties position, rotation, and scale.
* `Object3D` objects all are part of a scene graph.
	* Everything is added to the scene via `{object}.add()`. This means one can create arbitrary graphs.
* The scene is always the top level parent.
* Every object only has one parent. Each object can have multiple children.
* Order of rotations applied to an object matter. Translating and scaling are not affected by order of application.
* Internally, positions are represented by a matrix.
    * It is way faster to process these.
* All objects have 2 matricies.
	* Local Matrix: stores the local position, rotation, and scale information
	* World Matrix: stores the world-based position of the object.

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