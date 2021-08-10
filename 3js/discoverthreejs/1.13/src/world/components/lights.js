// import
import { AmbientLight, DirectionalLight, HemisphereLight } from '../../.././vendor/three/build/three.module.js';

// class
function createLights() {
	// create the hemisphere ambient light
	const ambientLight = new HemisphereLight(
		'white', 		     // bright sky color
		'darkslategrey',     // dim ground color
		3,                   // intensity
	);

	// create the directional light
	const directLight = new DirectionalLight('white', 1);

	// position the light
	// it will go:
	// * right
	// * up
	// * towards us
	// it shines from (10, 10, 10) to (0, 0, 0)
	directLight.position.set(10, 10, 10);

	// return 
	return { ambientLight, directLight };
}

// export
export { createLights };