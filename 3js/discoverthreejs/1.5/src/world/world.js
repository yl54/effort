// imports
import { createCamera } from './components/camera.js';
import { createCube } from './components/cube.js';
import { createLights } from './components/lights.js';
import { createScene } from './components/scene.js';
import { createRenderer } from './systems/renderer.js';
import { Resizer } from './systems/resizer.js';

/*
   Module scoped objects, not exposed elsewhere
   They kind of act like fields for the world class
   Javascript fields are public, theres no good support for private fields
   This will not work if there are two instances of the world class, 
   but there should only be one instance for most purposes.
*/
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
		container.append(renderer.domElement);

		const cube = createCube();
		const light = createLights();

		// {object}.add allows you to add any classes that is based off of `Object3D` to the graph
		scene.add(cube);

		// This leads to a different outcome from scene.add(). 
		// The positioning is based off of the local space of the cube.
		cube.add(light);

		const resizer = new Resizer(container, camera, renderer);
	}

	// render function
	// no parameters
	render() {
		renderer.render(scene, camera);
	}
}

// export world
export { World }