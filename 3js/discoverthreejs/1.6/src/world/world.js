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

		// Challenge Medium: 1
		// Disable antialiasing. Now, zoom in on the cube to get a better view of the aliasing artifact.
		// It makes it a little clearer to see the rugged edges.
		// cube.position.set(0, 0, 1);

		// Challenge Medium: 3
		// Still with AA disabled, use cube.position.x (horizontal movement) and cube.position.y (vertical movement) to zoom in on the right-hand corner of the cube.
		// It makes it a little clearer to see the rugged edges.
		// 9, weird things happen, it just goes to background
		// 8.95, a few surfaces are completely gone and its red.
		// cube.position.set(-1.3, -0.55, 8.9);

		// Challenge Medium: 2
		// Still with AA disabled, use camera.position.x (horizontal movement) and camera.position.y (vertical movement) to zoom in on the right-hand corner of the cube.
		// It makes it a little clearer to see the rugged edges.
		// 1.10, weird things happen, it just goes to background
		// 1.05, a few surfaces are completely gone and its red.
		camera.position.set(1.3, 0.55, 1.05);

		scene.add(cube, light);

		const resizer = new Resizer(container, camera, renderer);
		
		// customize the resize function
		// Challenge Easy: 4
		// Comment out the custom onResize hook in World.js and see what happens when you resize
		// Nothing really changes. I'm not sure if thats intended or not.
		resizer.onResize = () => {
			this.render();
		};
	}

	// render function
	// no parameters
	render() {
		renderer.render(scene, camera);
	}
}

// export world
export { World }