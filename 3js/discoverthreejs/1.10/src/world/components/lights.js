// import
import { DirectionalLight } from '../../.././vendor/three/build/three.module.js';

// class
function createLights() {
	// create the light
	const light = new DirectionalLight('white', 4);

	// position the light
	// it will go:
	// * right
	// * up
	// * towards us
	// it shines from (10, 10, 10) to (0, 0, 0)
	light.position.set(10, 10, 10);

	// return 
	return light;
}

// export
export { createLights };