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

	cube.rotation.set(-0.5, -0.1, 0.8);

	// Alter cube shape to be:
	// - wider
	// - shorter
	// - wider in depth
	cube.scale.set(2, 0.3, 6)

	// Challenge Easy: 1
	// Change the position, rotation, scale
	// cube.rotation.set(-3, -0.4, 5);
	// cube.scale.set(0.5, 0.8, 0.7);
	// cube.position.set(1, 0, 2);

	// return the mesh. its not a cube anymore
	return cube;
}

// export
export { createCube };