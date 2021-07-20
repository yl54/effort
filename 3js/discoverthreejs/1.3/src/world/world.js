// imports
import { createCamera } from './components/camera.js';
import { createCube } from './components/cube.js';
import { createScene } from './components/scene.js';
import { createRenderer } from './systems/renderer.js';
import { Resizer } from './systems/resizer.js';

// Module scoped objects, not exposed elsewhere
// They kind of act like fields for the world class
// Javascript fields are public, theres no good support for private fields
let camera;
let scene;
let renderer;

// World class
class World {
	// constructor
	// q: what is container? is it the html container?
	constructor(container) {
		camera = createCamera();
		scene = createScene();
		renderer = createRenderer();
	}

	// render function
	// no parameters
	render() {}
}

// export world
export { World }