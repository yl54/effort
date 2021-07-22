// imports

const setSize = (container, camera, renderer) => {
	// ------------- Update the camera ---------------

	// Set the camera's aspect ratio
	camera.aspect = container.clientWidth / container.clientHeight;

	// Update the frustum
	camera.updateProjectionMatrix();
		
	// ------------- Update the renderer ---------------
	// Update the size of the renderer and the canvas
	renderer.setSize(container.clientWidth, container.clientHeight);

	// set the pixel ratio
	renderer.setPixelRatio(window.devicePixelRatio);
};

// create function
class Resizer {
	constructor(container, camera, renderer) {
		setSize(container, camera, renderer);

		// add an event listener
		window.addEventListener('resize', () => {
			// set the window size if this event comes
			setSize(container, camera, renderer);

			// this function is set outside to render the scene again. 
			// this means that everytime the event listener is triggered,
			// the renderer will render the scene again.
			this.onResize();
		});
	}

	// declare the function here so that it can be customized outside
	onResize() {}
}

// export
export { Resizer };