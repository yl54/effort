// imports
import { World } from './world/world.js';

// main function
function main() {
	// Get a reference to the html elements, and keep them hidden to begin
	const container = document.querySelector('#scene-container');
	container.style.visibility = "hidden";
	const heading = document.querySelector('#heading');
	heading.style.visibility = "hidden";

	// Create an instance of the world app
	const world = new World(container);

	// Challenge Hard: 1
	// Add a button to trigger rendering the scene
	const button = document.querySelector('#render-button');

	button.addEventListener('click', () => {
		// Render the scene after the click occurs
		world.render();

		console.log("clicked");

		// Show the elements once clicked
		heading.style.visibility = "visible";
		container.style.visibility = "visible";
		button.style.visibility = "hidden";
		button.style.disabled = true;
	});

}

// call the main function
main();