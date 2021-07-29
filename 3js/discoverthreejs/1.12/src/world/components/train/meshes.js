// import
import { Mesh } from '../../../.././vendor/three/build/three.module.js';

import { createGeometries } from './geometries.js';
import { createMaterials } from './materials.js';

// create function
function createMeshes() {
	const geometries = createGeometries();
	const materials = createMaterials();

	// create the chimney mesh
	const chimney = new Mesh(geometries.chimney, materials.detail);
	chimney.position.set(-2, 1.9, 0);

	// create the cabin mesh
	const cabin = new Mesh(geometries.cabin, materials.body);
	cabin.position.set(1.5, 1.4, 0);

	// create the nose mesh
	const nose = new Mesh(geometries.nose, materials.body);
	nose.position.set(-1, 1, 0);
	nose.rotation.z = Math.PI / 2;

	// create the prototype wheel
	const smallWheelRear = new Mesh(geometries.wheel, materials.detail);
	smallWheelRear.position.y = 0.5;
	smallWheelRear.rotation.x = Math.PI / 2;

	// create 2 small wheels
	const smallWheelCenter = smallWheelRear.clone();
	smallWheelCenter.position.x = -1;

	const smallWheelFront = smallWheelRear.clone();
	smallWheelFront.position.x = -2;

	// create the 1 large wheel at the back
	const bigWheel = smallWheelRear.clone();
	bigWheel.position.set(1.5, 0.9, 0);
	bigWheel.scale.set(2, 1.25, 2);

	// return everything
	return {
		chimney,
		cabin,
		nose,
		smallWheelRear,
		smallWheelCenter,
		smallWheelFront,
		bigWheel,
	};
}

// export function
export { createMeshes };