// imports
import { World } from './world/world.js';

// main function
async function main() {
	// Get a reference to the container element
	const container = document.querySelector('#scene-container');

	// Create an instance of the world app
	const world = new World(container);

	// Call async setup steps for the world app
	await world.init();

	// Render the scene
	world.start();
}

// call the main function
// catch is a way to catch any error that pops up in the async method
main().catch((err) => {
	console.error(err);
});