// import
import {
	MeshStandardMaterial,
} from '../../.././vendor/three/build/three.module.js';

// create function
function createMaterials() {
	const body = new MeshStandardMaterial({
		color: 'firebrick',
		flatShading: true,
	});

	const detail = new MeshStandardMaterial({
		color: 'darkslategrey',
		flatShading: true,
	});

	return { body, detail };
}

// export function
export { createMaterials };