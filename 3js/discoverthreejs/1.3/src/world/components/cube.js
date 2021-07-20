// import
import { BoxBufferGeometry, Mesh, MeshBasicMaterial } from '../.././vendor/three/build/three.module.js';

// create function
function createCube() {
	// Create a geometry
	const geometry = new BoxBufferGeometry(2, 2, 2);

	// Create a material
	const material = new MeshBasicMaterial();

	// Create a Mesh 
	const cube = new Mesh(geometry, material);

	// return the mesh
	return cube;
}

// export
export { createCube };