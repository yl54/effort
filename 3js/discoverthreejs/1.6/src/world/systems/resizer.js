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

			// q: Why does this function need to be here? Without this, any resize is a black screen.
			this.onResize();
		});
	}

	onResize() {}
}

// export
export { Resizer };