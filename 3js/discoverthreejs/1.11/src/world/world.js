// imports
import { createCamera } from './components/camera.js';
import { createCube } from './components/cube.js';
import { createLights } from './components/lights.js';
import { createMeshGroup } from './components/meshgroup.js';
import { createScene } from './components/scene.js';
import { createControls } from './systems/controls.js';
import { Loop } from './systems/loop.js';
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
let loop;

// World class
class World {
	// constructor
	// q: what is container? is it the html container?
	constructor(container) {
		camera = createCamera();
		scene = createScene();
		renderer = createRenderer();
		loop = new Loop(camera, scene, renderer);
		container.append(renderer.domElement);

		const controls = createControls(camera, renderer.domElement);

		const meshGroup = createMeshGroup();
		const { ambientLight, directLight }  = createLights();

		// add the cube to the update list for the loop
		loop.updateables.push(meshGroup, controls);

		// {object}.add allows you to add any classes that is based off of `Object3D` to the graph
		scene.add(ambientLight, directLight, meshGroup);

		const resizer = new Resizer(container, camera, renderer);
	}

	// render function
	// no parameters
	render() {
		renderer.render(scene, camera);
	}

	start() {
		loop.start();
	}

	stop() {
		loop.stop();
	}
}

// export world
export { World }