# 1.8

## Summary
This is a threejs application that introduces the texture mapping.

## Terms
* `Texture Mapping` - The act of taking an image and stretching it over the surface of a 3D object. Like a well-fitting mask of sorts.
* `Image` - A 2D picture designed to be viewed by a human. A 2D array of colors.
* `Texture` - Specifically prepared data used for various purposes in 3D graphics. A 2D array of data, which includes colors as part of it. It can be referred to as a `Texture Map`.

## Information
* The basic material used in previous chapters is unrealistic. There are more realistic materials available.
* Artists can pick to represent features as either materials or geometry.
    * Usually, its cheaper to represent features as materials rather than geometry.
    * i.e hair could be geometry, but its so many verticies to process. Its likely more efficient to represent it as material.
* Usually, fewer objects is more performant + easier to work with.
    * q: Is it possible that geometries can be too complicated and monolithic (like programs)? The concern is if a geometry is too big to the point where its very difficult to change them.
* General q: Is it better for performance to have more, less complicated objects, or fewer, more complicated objects?
* Flat surfaces are easier to work with than curved surfaces.
* UV Mapping is a conversion (u, v) => (x, y, z).
    * Only corners are mapped, everything else is inferred.

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