// imports
import { createCamera } from './components/camera.js';
import { createCube } from './components/cube.js';
import { createRectangle } from './components/rectangle.js';
import { createScene } from './components/scene.js';
import { createSphere } from './components/sphere.js';
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
		scene.add(cube);

		// Challenge Medium: 2
		// add a second cube and move it
		const cube2 = createCube();
		scene.add(cube2);
		cube2.position.set(-2, -2, -2);

		console.log(scene);

		const rectangle = createRectangle();
		// scene.add(rectangle);

		// const sphere = createSphere();
		// scene.add(sphere);

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