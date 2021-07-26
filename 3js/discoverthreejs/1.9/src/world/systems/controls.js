// import
import { OrbitControls } from '../../.././vendor/three/examples/jsm/controls/OrbitControls.js';

// create
function createControls(camera, canvas) {
	const controls = new OrbitControls(camera, canvas);

	return controls;
}

// export
export { createControls };