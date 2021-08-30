// Challenge Medium: 1
// make a different shape from the cube

// import
import { SphereGeometry, Mesh, MeshBasicMaterial } from '../../.././vendor/three/build/three.module.js';

// create function
function createSphere() {
	// Create a geometry
	const geometry = new SphereGeometry();

	// Create a material
	const material = new MeshBasicMaterial();

	// Create a Mesh 
	const sphere = new Mesh(geometry, material);

	// return the mesh
	return sphere;
}

// export
export { createSphere };