// imports
import { World } from './world/world.js';

// main function
function main() {
	// Get a reference to the container element
	const container = document.querySelector('#scene-container');

	// Create an instance of the world app
	const world = new World(container);

	// Render the scene
	world.render();
}

// call the main function
main();