// import
import { BoxBufferGeometry, Mesh, MeshPhongMaterial, MeshStandardMaterial } from '../../.././vendor/three/build/three.module.js';

// Challenge Easy: 1
// Change the color of the material
const spec = {
	color: 'aqua',
}

// create function
function createCube() {
	// Create a geometry
	const geometry = new BoxBufferGeometry(2, 2, 2);

	// Create a MeshStandardMaterial
	const material = new MeshStandardMaterial(spec);

	// Challenge Medium: 2
	// Use a different material
	// const material = new MeshPhongMaterial(spec);

	// Create a Mesh 
	const cube = new Mesh(geometry, material);

	cube.rotation.set(-0.5, -0.1, 0.8);

	// return the mesh
	return cube;
}

// export
export { createCube };