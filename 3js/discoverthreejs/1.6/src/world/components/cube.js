// import
import { BoxBufferGeometry, Mesh, MeshStandardMaterial } from '../../.././vendor/three/build/three.module.js';

const spec = {
	color: 'purple',
}

// create function
function createCube() {
	// Create a geometry
	const geometry = new BoxBufferGeometry(2, 2, 2);

	// Create a MeshStandardMaterial
	const material = new MeshStandardMaterial(spec);

	// Create a Mesh 
	const cube = new Mesh(geometry, material);

	// Challenge Easy: 2
	// Rotate the cube until the edges are vertical and horizontal.
	// Its hard to tell the difference between anti-alias + no anti-alias
	cube.rotation.set(-0.5, -0.1, 0.8);
	// cube.rotation.set(0, 0, 0);

	// return the mesh. its not a cube anymore
	return cube;
}

// export
export { createCube };