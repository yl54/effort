// import
import { WebGLRenderer } from '../.././vendor/three/build/three.module.js';

// class
function createRenderer() {
	const renderer = new WebGLRenderer();

	return renderer;
}

// export
export { createRenderer };