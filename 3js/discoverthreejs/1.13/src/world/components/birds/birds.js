// imports
import { setupModel } from './setupModel.js';
import { GLTFLoader } from '../../../.././vendor/three/examples/jsm/loaders/GLTFLoader.js';

// load function
async function loadBirds() {
	// create a loader
	const loader = new GLTFLoader();

	// use promise all to async load all of the models at once
	// as opposed to await each individual load
	const [parrotData, flamingoData, storkData] = await Promise.all([
		loader.loadAsync('/assets/models/Parrot.glb'),
		loader.loadAsync('/assets/models/Flamingo.glb'),
		loader.loadAsync('/assets/models/Stork.glb'),
	]);

	console.log("loaded bird", parrotData);

	const parrot = setupModel(parrotData);
	parrot.position.set(100, 0, 0);

	const flamingo = setupModel(flamingoData);
	flamingo.position.set(0, 100, 0);

	const stork = setupModel(storkData);
	stork.position.set(0, 0, 100);

	return { parrot, flamingo, stork }
}

// export
export { loadBirds };