// import
import { WebGLRenderer } from '../../.././vendor/three/build/three.module.js';

// NOTE: A renderer cannot be changed while its in use midway.
const spec = {
	antialias: true
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