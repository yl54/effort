// import
import { DirectionalLight } from '../../.././vendor/three/build/three.module.js';


// class
function createLights() {
	// Challenge Easy: 1
	// Change the color of the light
	// create the light
	const light = new DirectionalLight('gray', 5);

	// Challenge Easy: 1
	// Change the position of the light
	// position the light
	// it will go:
	// * right
	// * up
	// * towards us
	// it shines from (10, 20, 5) to (0, 0, 0)
	light.position.set(10, 20, 5);

	// return 
	return light;
}

// export
export { createLights };