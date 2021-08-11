// import
import {
	Group,
	MathUtils,
	Mesh,
	MeshStandardMaterial,
	SphereBufferGeometry,
} from '../../.././vendor/three/build/three.module.js';

// radiansPerSecond is the radians to rotate per second
// the goal is to have the cube rotate thirty degrees per second at any FPS.
// If you want to speed up rotation, increase the radians per second.
// If you want to slow down rotation, decrease the radians per second.
const radiansPerSecond = MathUtils.degToRad(30);

// create mesh group
function createMeshGroup() {
	// create a group
	const group = new Group();

	// create a geometry
	const geometry = new SphereBufferGeometry(0.1, 8, 8);

	// create a material
	const material = new MeshStandardMaterial({
		color: 'indigo',
	});

	// create the mesh
	const mesh = new Mesh(geometry, material);

	// add the mesh to the group
	group.add(mesh);

	// clone the mesh 20 times and add to the group
	// We want to position the clones in a circle around the main sphere
	for (let i = 0; i < 1; i += 0.05) {
		const cl = mesh.clone();

		// x = cos(2 * pi * i)
		cl.position.x = Math.cos(2 * Math.PI * i);
		
		// y = sin(2 * pi * i)
		cl.position.y = Math.sin(2 * Math.PI * i);

		// This will make each individual object be at a different scale.
		// The subsequent spheres will be larger.
		cl.scale.multiplyScalar(0.01 + i);

		group.add(cl);
	}

	// update the tick function
	group.tick = (delta) => {
		// make the group of spheres rotate at 30 fps
		group.rotation.z -= delta * radiansPerSecond;
	};

	// scale the group objects up
	group.scale.multiplyScalar(2);

	// return the group
	return group;
}

// export
export { createMeshGroup };