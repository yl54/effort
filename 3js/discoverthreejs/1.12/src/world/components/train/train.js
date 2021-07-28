// import
import { Group } from '../../../.././vendor/three/build/three.module.js';

import { createMeshes } from './meshes.js';
// create function
class Train extends Group {
	constructor() {
		// Call the constructor for the class it inherits from
		super();

		this.meshes = createMeshes();
	}
}

// export function
export { Train };