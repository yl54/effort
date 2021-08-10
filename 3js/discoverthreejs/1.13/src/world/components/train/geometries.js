// import
import { 
	BoxBufferGeometry,
	CylinderBufferGeometry,
} from '../../../.././vendor/three/build/three.module.js';

// create function
function createGeometries() {
	// create a cabin
	const cabin = new BoxBufferGeometry(
		2,    // length 
		2.25, // width
		1.5   // height
	);

	// create a nose
	// this is a cylinder
	const nose = new CylinderBufferGeometry(
		0.75, // top radius
		0.75, // bottom radius
		3,    // height
		12    // radial segments
	);

	// create a wheel
	// this is a cylinder
	const wheel = new CylinderBufferGeometry(
		0.4,  // top radius
		0.4,  // bottom radius
		1.75, // height
		16    // radial segments
	);

	// create a chimney
	// this is a cone
	const chimney = new CylinderBufferGeometry(
		0.3, // top radius
		0.1, // bottom radius
		0.5  // height
		// default value
	)

	// return all of the geometries
	return {
		cabin,
		nose,
		wheel,
		chimney
	};
}

// export function
export { createGeometries };