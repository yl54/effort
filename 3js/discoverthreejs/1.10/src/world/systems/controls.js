// import
import { OrbitControls } from '../../.././vendor/three/examples/jsm/controls/OrbitControls.js';

// create
function createControls(camera, canvas) {
	const controls = new OrbitControls(camera, canvas);

	// Set the orbit point of the controls. The default is (0, 0, 0).
	controls.target.set(1, 1, 1);

	controls.enableDamping = true;

	// Update the controls with each tick. 
	// q: what actually gets updated?
	controls.tick = () => controls.update();

	// code to enable/disable controls
	/*
	controls.enabled = true;
	*/ 

	// code to enable/disable individual modes
	/*
	controls.enableRotate = true;
	controls.enableZoom = true;
	controls.enablePan = true;
	*/

	return controls;
}

// export
export { createControls };