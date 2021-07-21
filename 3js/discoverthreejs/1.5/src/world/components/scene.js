// import
import { Color, Scene } from '../../.././vendor/three/build/three.module.js';

// create function
function createScene() {
	const scene = new Scene();

	scene.background = new Color('red'); 

	return scene;
}

// export
export { createScene };