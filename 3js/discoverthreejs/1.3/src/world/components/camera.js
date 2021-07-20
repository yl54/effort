// import
import { PerspectiveCamera } from '../.././vendor/three/build/three.module.js';

// create camera
function createCamera(fov, nearClip, farClip) {
	// Set the camera parameters
	const fov = 35;
	const aspect = 1;
	const near = nearClip;
	const far = farClip;

	// Create the camera
 	const camera = new PerspectiveCamera(fov, aspect, near, far);

 	// Set the camera location
 	camera.position.set(0, 0, 10);

	// return camera
	return camera;
}

// export
export { createCamera };