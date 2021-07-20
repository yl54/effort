// imports

// create function
class Resizer {
	constructor(container, camera, renderer) {
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
	}
}

// export
export { Resizer };