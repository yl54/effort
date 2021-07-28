// import
import { Mesh } from '../../../.././vendor/three/build/three.module.js';

import { createGeometries } from './geometries.js';
import { createMaterials } from './materials.js';

// create function
function createMesh() {
	const geometries = createGeometries();
	const materials = createMaterials();

	
}

// export function
export { createMesh };