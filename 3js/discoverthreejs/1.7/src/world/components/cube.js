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

	// Any update to the cube will execute this function
	cube.tick = () => {
		cube.rotation.z += 0.01;
		cube.rotation.y += 0.01;
		cube.rotation.x += 0.01;
	};

	// return the mesh. its not a cube anymore
	return cube;
}

// export
export { createCube };