// import
import { WebGLRenderer } from '../../.././vendor/three/build/three.module.js';

// class
function createRenderer() {
	const renderer = new WebGLRenderer();

	// turn on physically correct lighting
	renderer.physicallyCorrectLights = true;

	return renderer;
}

// export
export { createRenderer };