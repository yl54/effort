// Challenge Medium: 1
// make a different shape from the cube

// import
import { BoxBufferGeometry, Mesh, MeshBasicMaterial } from '../../.././vendor/three/build/three.module.js';


// create function
function createRectangle() {
	// Create a geometry
	const geometry = new BoxBufferGeometry(2, 3, 2);

	// Create a material
	const material = new MeshBasicMaterial();

	// Create a Mesh 
	const rectangle = new Mesh(geometry, material);

	// return the mesh
	return rectangle;
}

// export
export { createRectangle };