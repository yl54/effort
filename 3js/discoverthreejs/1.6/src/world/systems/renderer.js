// import
import { WebGLRenderer } from '../../.././vendor/three/build/three.module.js';

// NOTE: A renderer cannot be changed while its in use midway.
// Challenge Easy: 1
// Enable and disable AA and compare the difference.
// When anti-alising is disabled, the edges are not smooth.
const spec = {
	antialias: false
}

// class
function createRenderer() {
	const renderer = new WebGLRenderer(spec);

	// turn on physically correct lighting
	renderer.physicallyCorrectLights = true;

	return renderer;
}

// export
export { createRenderer };