// imports
import { setupModel } from './setupModel.js';
import { GLTFLoader } from '../../../.././vendor/three/examples/jsm/loaders/GLTFLoader.js';

// load function
async function loadBirds() {
	// create a loader
	const loader = new GLTFLoader();

	// get the parrot data from the loader
	const parrotData = await loader.loadAsync('/assets/models/Parrot.glb');

	console.log("loaded bird", parrotData);

	const parrot = setupModel(parrotData);

	return { parrot }

}

// export
export { loadBirds };