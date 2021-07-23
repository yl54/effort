// import
import { BoxBufferGeometry, MathUtils, Mesh, MeshStandardMaterial } from '../../.././vendor/three/build/three.module.js';

const spec = {
	color: 'purple',
}

// radiansPerSecond is the radians to rotate per second
// the goal is to have the cube rotate thirty degrees per second at any FPS.
// If you want to speed up rotation, increase the radians per second.
// If you want to slow down rotation, decrease the radians per second.
const radiansPerSecond = MathUtils.degToRad(30);

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
	// Over 1 second, delta's total will be 1 second.
	// Over 1 second, a rough formula may look like this:
	// (radiansPerSecond * 0.016) + (radiansPerSecond * 0.016) + ...  = radiansPerSecond
	cube.tick = (delta) => {
		cube.rotation.z += radiansPerSecond * delta;
		cube.rotation.y += radiansPerSecond * delta;
		cube.rotation.x += radiansPerSecond * delta;
	};

	// return the mesh. its not a cube anymore
	return cube;
}

// export
export { createCube };